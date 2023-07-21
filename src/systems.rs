use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
    sprite::Mesh2dHandle,
};

#[cfg(feature = "rapier")]
use bevy_rapier2d::prelude::*;

use bevy_variable_property::prelude::*;

use smallvec::SmallVec;
use thiserror::Error;

#[cfg(not(feature = "rapier"))]
use crate::phys::{Damping, Velocity};

use crate::{
    components::*,
    despawn::{DespawnMaterial, NoDespawnAnimation},
    events::DespawnParticlesEvent,
    utils::{angle_between3, float32x3_sub, float32x3_triangle_centroid},
};

#[derive(Debug)]
struct ImageParams {
    // The image to use in the shader.
    pub image_handle: Handle<Image>,

    // Top-left bound offset of the image, primarily for sprite sheets
    pub offset: Vec2,

    // The size of the section of the image to pull from.
    pub input_size: Vec2,

    // The size of the entire source texture
    pub texture_size: Vec2,

    // The custom_size set by the parent, if applicable.
    pub custom_size: Option<Vec2>,
}

/// Spawns death particles by creating a particles with a shader that pulls a small portion of the original texture
pub(crate) fn handle_despawn_particles_event(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    atlases: Res<Assets<TextureAtlas>>,
    global_transforms: Query<&GlobalTransform>,
    mut despawn_materials: ResMut<Assets<DespawnMaterial>>,
    sprites: Query<(&Sprite, &Handle<Image>)>,
    tass: Query<(&TextureAtlasSprite, &Handle<TextureAtlas>)>,
    mesh_components: Query<(&Mesh2dHandle, Option<&Handle<ColorMaterial>>)>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut despawn_particles_event_reader: EventReader<DespawnParticlesEvent>,
    no_death_animations: Query<&NoDespawnAnimation>,
    velocities: Query<&Velocity>,
    despawn_mesh_overrides: Query<&DespawnMeshOverride>,
) {
    for DespawnParticlesEvent {
        entity,
        linvel,
        linvel_addtl,
        angvel,
        ignore_parent_phys,
        lifetime,
        linear_damping,
        angular_damping,
        mass,
        shrink,
        fade,
        mesh_override: event_mesh_override,
        target_num_particles,
        gray
    } in despawn_particles_event_reader.iter()
    {
        let target_num_particles = target_num_particles.get_value();

        let gray: u32 = gray.then(|| 1).unwrap_or(0); // Need to convert for shader

        // Use closures so we don't have to re-do the if statement for every single particle.
        // This assumes the no-op actually gets optimized out, which is may not..
        let shrink_spawn_func = if *shrink {
            |entity_cmds: &mut EntityCommands| {
                entity_cmds.insert(ShrinkingDespawnParticle);
            }
        } else {
            |_entity_cmds: &mut EntityCommands| {}
        };

        let fade_spawn_func = if *fade {
            |entity_cmds: &mut EntityCommands| {
                entity_cmds.insert(FadingDespawnParticle);
            }
        } else {
            |_entity_cmds: &mut EntityCommands| {}
        };

        if let Some(mut entity_commands) = commands.get_entity(*entity) {
            entity_commands.despawn();
            // Now spawn the death animation, if possible
            if no_death_animations.get(*entity).is_ok() {
                // We ignore death animations for this object.
                continue;
            }

            let (mesh_handle, maybe_image_params, maybe_color_material) = if let Ok((
                sprite,
                image_handle,
            )) =
                sprites.get(*entity)
            {
                let image_size = if let Some(image) = images.get(&image_handle) {
                    image.size()
                } else {
                    warn!(
                        "Could not get image data to generate despawn particles for entity {:?}",
                        entity
                    );
                    continue;
                };
                let mesh = shape::Quad::new(image_size);

                (
                    meshes.add(mesh.into()).into(),
                    Some(ImageParams {
                        offset: Vec2::ZERO,
                        image_handle: image_handle.clone(),
                        input_size: image_size,
                        texture_size: image_size,
                        custom_size: sprite.custom_size,
                    }),
                    None,
                )
            } else if let Ok((tas, ta_handle)) = tass.get(*entity) {
                if let Some(atlas) = atlases.get(ta_handle) {
                    if let Some(rect) = atlas.textures.get(tas.index) {
                        if let Some(image) = images.get(&atlas.texture) {
                            let input_size = Vec2::new(rect.width(), rect.height());
                            let mesh = shape::Quad::new(input_size);
                            (
                                meshes.add(mesh.into()).into(),
                                Some(ImageParams {
                                    offset: rect.min,
                                    image_handle: atlas.texture.clone(),
                                    input_size,
                                    texture_size: image.size(),
                                    custom_size: tas.custom_size,
                                }),
                                None,
                            )
                        } else {
                            warn!("Cannot generate despawn particles, invalid image handle for texture atlas on entity: {:?}", entity);
                            continue;
                        }
                    } else {
                        warn!("Cannot generate despawn particles, invalid texture atlas index for entity: {:?}", entity);
                        continue;
                    }
                } else {
                    warn!(
                        "Could not get atlas to generate particles for entity {:?}",
                        entity
                    );
                    continue;
                }
            } else if let Ok((mesh_handle, maybe_color_material)) = mesh_components.get(*entity) {
                let base_color = maybe_color_material
                    .and_then(|handle| color_materials.get(handle))
                    .and_then(|material| Some(material.color))
                    .unwrap_or(Color::GRAY);
                let mixed_shade =
                    base_color.r() * 0.299 + base_color.g() * 0.587 + base_color.b() * 0.114;
                let mixed_color =
                    Color::rgba(mixed_shade, mixed_shade, mixed_shade, base_color.a());
                (
                    mesh_handle.clone(),
                    None,
                    Some(color_materials.add(ColorMaterial::from(mixed_color))),
                )
            } else {
                warn!(
                    "Entity {:?} does not have a mesh or sprite to use for particles",
                    entity
                );
                continue;
            };

            // Find which mesh to use.
            let mesh_handle = event_mesh_override
                .clone()
                .or_else(|| {
                    despawn_mesh_overrides
                        .get(*entity)
                        .and_then(|c| Ok(c.0.clone()))
                        .ok()
                })
                .unwrap_or(mesh_handle.0);

            // Break the mesh into smaller triangles
            let triangle_meshes = if let Some(mut mesh) = meshes.get(&mesh_handle).cloned() {
                if let PrimitiveTopology::TriangleList = mesh.primitive_topology() {
                    let vertices = if let Some(vertices) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                    {
                        match vertices {
                            // We may want to implement this at some point. Add 0.0 Z value and
                            // create a new Float32x3 vertices.
                            /*VertexAttributeValues::Float32x2(vertices) => {
                                vertices
                            }*/
                            VertexAttributeValues::Float32x3(vertices) => {
                                // For easy math using Vec3's builtin methods, we want to convert
                                // these over.
                                vertices
                                    .iter()
                                    .map(|vertex| Vec3::from(*vertex))
                                    .collect::<Vec<_>>()
                            }

                            _ => {
                                warn!("Cannot create despawn particles, unexpected vertex attribute for mesh on entity {:?}", entity);
                                continue;
                            }
                        }
                    } else {
                        warn!(
                            "Cannot create despawn particles for a mesh with no vertices: {:?}",
                            entity
                        );
                        continue;
                    };

                    if let Some(indices) = mesh.indices() {
                        // We have indices so build triangles from that and the vertices
                        if indices.len() % 3 != 0 {
                            warn!("Cannot create despawn particles, mesh has invalid indices on entity {:?}", entity);
                            continue;
                        }
                    } else {
                        // We have no indices, so add them by hand and return the number of
                        // triangles after
                        if vertices.len() % 3 != 0 {
                            warn!("Cannot create despawn particles, mesh has invalid vertices on entity {:?}", entity);
                            continue;
                        }
                        mesh.set_indices(Some(Indices::U32(
                            (0..(vertices.len() as u32)).collect(),
                        )));
                    };

                    // Break down the triangles into individual meshes
                    let meshes = match split_mesh(mesh, target_num_particles) {
                        Ok(meshes) => meshes,
                        Err(e) => {
                            warn!(
                                "Failed to split mesh for despawn particles: {}; entity = {:?}",
                                e, entity
                            );
                            continue;
                        }
                    };

                    // Re-center the triangles around the origin, saving that offset for the
                    // Transform
                    // We can assume every mesh has TriangleList topology and proper indices.

                    meshes
                        .into_iter()
                        .map(|mut mesh| {
                            // These unwraps are guaranteed safe
                            let vertices: [[f32; 3]; 3] = mesh
                                .attribute(Mesh::ATTRIBUTE_POSITION)
                                .unwrap()
                                .as_float3()
                                .unwrap()
                                .try_into()
                                .unwrap();

                            // Get the centroid of the triangle, we will use this to translate this
                            // mesh to the origin
                            let centroid = float32x3_triangle_centroid(vertices);

                            // Translate the triangle around the origin point using the centroid.
                            // Collect into a Vec since it will be converted to this for the mesh
                            // anyway.
                            let new_vertices = vertices
                                .iter()
                                .map(|v| float32x3_sub(*v, centroid))
                                .collect::<Vec<_>>();

                            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_vertices);
                            (mesh, Vec3::from(centroid))
                        })
                        .collect::<Vec<_>>()
                } else {
                    warn!("Cannot create despawn particles for a mesh that does not use a TriangleList topology: {:?}", entity);
                    continue;
                }
            } else {
                warn!("Cannot create despawn particles, mesh handle on entity {:?} is no longer valid", entity);
                continue;
            };

            if let Ok(orig_transform) = global_transforms.get(*entity) {
                let orig_transform: Transform = (*orig_transform).into();
                let center_point = orig_transform.translation;

                // scale to apply to each new mesh
                let scale = orig_transform.scale
                    * maybe_image_params
                        .as_ref()
                        .and_then(|params| {
                            params
                                .custom_size
                                .and_then(|size| Some((size / params.input_size).extend(1.0)))
                        })
                        .unwrap_or(Vec3::ONE);

                for (mesh, offset) in triangle_meshes {
                    let addtl_translation = maybe_image_params
                        .as_ref()
                        .and_then(|p| p.custom_size.and_then(|size| Some(size / p.input_size)))
                        .unwrap_or(Vec2::ONE);
                    let translation = (center_point
                        + orig_transform.rotation.normalize().mul_vec3(offset))
                        * orig_transform.scale
                        * addtl_translation.extend(1.0);
                    let angle = angle_between3(center_point, translation);
                    let parent_velocity = velocities.get(*entity).copied().unwrap_or_default();

                    let particle_transform = Transform {
                        translation,
                        rotation: orig_transform.rotation,
                        scale,
                    };

                    let vel_scalar = linvel.get_value();
                    let velocity = Vec2::new(vel_scalar * angle.sin(), vel_scalar * angle.cos())
                        + if *ignore_parent_phys {
                            Vec2::ZERO
                        } else {
                            // Use the parent's last known angvel to calculate additional linear
                            // velocity
                            let perp_angle = angle - (std::f32::consts::PI / 2.0);
                            let radius = center_point.distance(translation);
                            let total_velocity_from_angvel = radius * parent_velocity.angvel;
                            let additional_velocity_from_angvel = Vec2::new(
                                total_velocity_from_angvel * perp_angle.sin(),
                                total_velocity_from_angvel * perp_angle.cos(),
                            );
                            parent_velocity.linvel + additional_velocity_from_angvel
                        }
                        + linvel_addtl.get_value();

                    let mut entity_cmds = commands.spawn((
                        DespawnParticleBundle {
                            despawn_particle: DespawnParticle::new(lifetime.get_value()),
                            velocity: Velocity {
                                linvel: velocity,
                                angvel: angvel.get_value(),
                            },
                            damping: Damping {
                                linear_damping: linear_damping.get_value(),
                                angular_damping: angular_damping.get_value(),
                            },
                            #[cfg(not(feature = "rapier"))]
                            mass: mass.get_value().into(),
                            #[cfg(feature = "rapier")]
                            mass: AdditionalMassProperties::Mass(mass.get_value()),
                            ..default()
                        },
                        Mesh2dHandle::from(meshes.add(mesh)),
                        SpatialBundle {
                            transform: particle_transform,
                            ..default()
                        },
                    ));

                    if let Some(image_params) = maybe_image_params.as_ref() {
                        // We have a texture
                        let material = despawn_materials.add(DespawnMaterial {
                            alpha: 1.0,
                            source_image: Some(image_params.image_handle.clone()),
                            offset: (image_params.offset / image_params.texture_size),
                            size: (image_params.input_size / image_params.texture_size),
                            gray,
                        });
                        entity_cmds.insert(material);
                    } else if let Some(color_material_handle) = maybe_color_material.clone() {
                        // We have no texture, just use color materials
                        entity_cmds.insert(color_material_handle.clone());
                        entity_cmds.insert(OriginalAlpha(
                            color_materials
                                .get(&color_material_handle)
                                .and_then(|material| Some(material.color.a()))
                                .unwrap_or(1.0),
                        ));
                    }

                    shrink_spawn_func(&mut entity_cmds);
                    fade_spawn_func(&mut entity_cmds);
                }
            }
        }
    }
}

pub(crate) fn handle_despawn_particle(
    mut despawn_particles: Query<(
        Entity,
        AnyOf<(
            &Handle<DespawnMaterial>,
            (&Handle<ColorMaterial>, &OriginalAlpha),
        )>,
        &mut DespawnParticle,
        &mut Transform,
        Option<&ShrinkingDespawnParticle>,
        Option<&FadingDespawnParticle>,
    )>,
    mut despawn_materials: ResMut<Assets<DespawnMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (
        entity,
        (maybe_despawn_material_handle, maybe_color_material_handle_and_alpha),
        mut despawn_particle,
        mut transform,
        maybe_shrink,
        maybe_fade,
    ) in despawn_particles.iter_mut()
    {
        despawn_particle.lifetime.tick(time.delta());
        if despawn_particle.lifetime.finished() {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
        let percent = despawn_particle.lifetime.percent_left();
        if let Some(mut despawn_material) = maybe_fade
            .and_then(|_| maybe_despawn_material_handle)
            .and_then(|handle| despawn_materials.get_mut(handle))
        {
            despawn_material.alpha = percent;
        } else if let Some((color_material, original_alpha)) = maybe_color_material_handle_and_alpha
            .and_then(|(handle, a)| color_materials.get_mut(handle).zip(Some(a)))
        {
            color_material.color.set_a(original_alpha.0 * percent);
        }
        if maybe_shrink.is_some() {
            transform.scale = Vec3::splat(percent);
        }
    }
}

#[derive(Error, Debug)]
pub enum SplitMeshError {
    #[error("Encountered an unexepcted PrimitiveTopology, expected PrimitiveTopology")]
    UnexpectedPrimitiveTopology,

    #[error("Unexpected number of TriangleList indices: {0}, should be divisible by 3")]
    UnexpectedIndexCount(usize),

    #[error("Unexpected format for position attribute of mesh, expected Float32x3")]
    UnexpectedPositionAttributeFormat,

    #[error("Unexpected format for uv attribute of mesh, expected Float32x2")]
    UnexpectedUvAttributeFormat,

    #[error("Mesh has no vertices")]
    MissingVertices,

    #[error("Mesh has no uvs")]
    MissingUvs,
}

pub fn split_mesh(mut mesh: Mesh, target_count: usize) -> Result<Vec<Mesh>, SplitMeshError> {
    if let PrimitiveTopology::TriangleList = mesh.primitive_topology() {
        let vertices = if let Some(vertices) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
            match vertices {
                // We may want to implement this at some point. Add 0.0 Z value and
                // create a new Float32x3 vertices.
                /*VertexAttributeValues::Float32x2(vertices) => {
                    vertices
                }*/
                VertexAttributeValues::Float32x3(vertices) => {
                    // For easy math using Vec3's builtin methods, we want to convert
                    // these over.
                    vertices
                        .iter()
                        .map(|vertex| Vec3::from(*vertex))
                        .collect::<Vec<_>>()
                }

                _ => {
                    return Err(SplitMeshError::UnexpectedPositionAttributeFormat);
                }
            }
        } else {
            return Err(SplitMeshError::MissingVertices);
        };

        let normals = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            .and_then(|normals| normals.as_float3())
            .map(|normals| normals.to_vec())
            .unwrap_or((0..vertices.len()).map(|_| [0.0, 0.0, 1.0]).collect());
             

        let indices = if let Some(indices) = mesh
            .indices()
            .and_then(|indices| Some(indices.iter().collect::<Vec<_>>()))
        {
            if indices.len() % 3 != 0 {
                return Err(SplitMeshError::UnexpectedIndexCount(indices.len()));
            }
            indices
        } else {
            // No indices provided, add them in
            let indices = (0..vertices.len()).collect::<Vec<_>>();
            mesh.set_indices(Some(Indices::U32(
                indices.iter().map(|i| *i as u32).collect::<Vec<_>>(),
            )));
            indices
        };

        // Get the UVs
        let uvs = if let Some(uvs) = mesh.attribute(Mesh::ATTRIBUTE_UV_0) {
            if let VertexAttributeValues::Float32x2(uvs) = uvs {
                uvs
            } else {
                return Err(SplitMeshError::UnexpectedUvAttributeFormat);
            }
        } else {
            return Err(SplitMeshError::MissingUvs);
        };

        let initial_meshes = indices
            .as_slice()
            .chunks(3)
            .map(|indices| {
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    indices.iter().map(|idx| vertices[*idx]).collect::<Vec<_>>(),
                );
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_UV_0,
                    indices.iter().map(|idx| uvs[*idx]).collect::<Vec<_>>(),
                );
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    indices.iter().map(|idx| normals[*idx]).collect::<Vec<_>>(),
                );

                mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
                mesh
            })
            .collect::<Vec<_>>();
        let num_triangles = initial_meshes.len();

        // This could be simplified if we just clamp the depth to [0.0, f32::MAX] and always call
        // split_mesh_inner, but we save a lot of extra processing if we just do this one if
        // statement, so I think this will be much more efficient in cases where we already have enough
        // triangles and only very slightly less efficient in cases where we do have to call it.
        if num_triangles < target_count {
            // We need to break the triangles down further.
            let depth = f32::log2(target_count as f32 / num_triangles as f32).ceil() as usize;
            let mut final_meshes = Vec::with_capacity(num_triangles * 2usize.pow(depth as u32));
            for mesh in initial_meshes {
                split_mesh_inner(mesh, depth, &mut final_meshes);
            }

            Ok(final_meshes)
        } else {
            // We have enough meshes, so just return them.
            Ok(initial_meshes)
        }
    } else {
        Err(SplitMeshError::UnexpectedPrimitiveTopology)
    }
}

// Mesh is assumed to have a TriangleList topology with a valid number of indices and vertices
// Will try to break the given mesh into at least target_count meshes.
// Mesh is also assumed to be a single Triangle
fn split_mesh_inner(mesh: Mesh, depth: usize, output: &mut Vec<Mesh>) {
    if depth == 0 {
        // Re-center the triangle around origin, and use that translation as the offset.
        //let p_center = v.iter().fold(Vec3::new(), |acc, v| acc + v
        output.push(mesh);
    } else {
        // Get the vertices and uvs, it is assumed this is checked prior.
        // Convert them here so they do not have to be converted more than once below.
        let raw_vertices = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap()
            .iter()
            .map(|v| Vec3::from(*v))
            .collect::<Vec<_>>();
        let uvs = if let VertexAttributeValues::Float32x2(uvs) =
            mesh.attribute(Mesh::ATTRIBUTE_UV_0).unwrap()
        {
            uvs.iter().map(|v| Vec2::from(*v)).collect::<Vec<_>>()
        } else {
            // This should never occur.
            warn!("Unexpected type for UV_0 attribute");
            return;
        };

        let indices = mesh
            .indices()
            .unwrap()
            .iter()
            .collect::<SmallVec<[usize; 3]>>();
        let v = [
            raw_vertices[indices[0]],
            raw_vertices[indices[1]],
            raw_vertices[indices[2]],
        ];

        if depth == 1 {
            let sides = [
                v[1].distance(v[2]),
                v[2].distance(v[0]),
                v[0].distance(v[1]),
            ];

            // Get the idx of the point across from the longest side
            let longest_idx =
                sides.iter().enumerate().fold(
                    0usize,
                    |acc, (idx, v)| {
                        if *v >= sides[acc] {
                            idx
                        } else {
                            acc
                        }
                    },
                );

            // Get the halfway point of this longest side, which is between the two other points
            let (p_mid, uv_mid) =
                (0..3)
                    .into_iter()
                    .fold((Vec3::ZERO, Vec2::ZERO), |acc, curr_idx| {
                        if longest_idx == curr_idx {
                            // Skip, we are ignoring our selected index
                            acc
                        } else {
                            (acc.0 + v[curr_idx], acc.1 + uvs[curr_idx])
                        }
                    });

            let (p_mid, uv_mid) = (p_mid / 2.0, uv_mid / 2.0);

            // Create the two new triangles
            for idx in (0..3).filter(|idx| *idx != longest_idx) {
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
                );
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    [v[longest_idx], p_mid, v[idx]].to_vec(),
                );
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_UV_0,
                    [uvs[longest_idx], uv_mid, uvs[idx]].to_vec(),
                );
                mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
                split_mesh_inner(mesh, depth - 1, output);
            }
        } else {
            // depth is >= 2
            // For cleaner breaks, we want to split each triangle into 4 equal triangles by
            // connecting each side's midpoint.

            // idx of 0 corresponds to the midpoint of the side opposite of that point
            let mps = [
                (v[1] + v[2]) / 2.0,
                (v[2] + v[0]) / 2.0,
                (v[0] + v[1]) / 2.0,
            ];

            let mps_uvs = [
                (uvs[1] + uvs[2]) / 2.0,
                (uvs[2] + uvs[0]) / 2.0,
                (uvs[0] + uvs[1]) / 2.0,
            ];

            for (vertices, uvs) in [
                (
                    vec![v[0], mps[1], mps[2]],
                    vec![uvs[0], mps_uvs[1], mps_uvs[2]],
                ),
                (
                    vec![v[1], mps[0], mps[2]],
                    vec![uvs[1], mps_uvs[0], mps_uvs[2]],
                ),
                (
                    vec![v[2], mps[0], mps[1]],
                    vec![uvs[2], mps_uvs[0], mps_uvs[1]],
                ),
                (
                    vec![mps[0], mps[1], mps[2]],
                    vec![mps_uvs[0], mps_uvs[1], mps_uvs[2]],
                ),
            ] {
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_NORMAL,
                    vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
                );
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));

                // depth - 2 here since we broke 1 triangle into 4 instead of just 2.
                split_mesh_inner(mesh, depth - 2, output);
            }
        }
    }
}
