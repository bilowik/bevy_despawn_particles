use bevy::{prelude::*, sprite::Mesh2dHandle};

#[cfg(feature = "rapier")]
use bevy_rapier2d::prelude::*;

use bevy_variable_property::prelude::*;

#[cfg(not(feature = "rapier"))]
use crate::phys::Velocity;

use crate::{
    components::*,
    events::DespawnParticlesEvent,
    despawn::{DespawnMaterial, NoDespawnAnimation},
    utils::angle_between3,
};

use rand::Rng;

const DEATH_VEL: f32 = 100.0;



/// Spawns death particles by creating a particles with a shader that pulls a small portion of the original texture 
pub fn handle_despawn_particles_event(
    mut commands: Commands,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    atlases: Res<Assets<TextureAtlas>>,
    global_transforms: Query<&GlobalTransform>,
    mut despawn_materials: ResMut<Assets<DespawnMaterial>>,
    image_handles: Query<&Handle<Image>>,
    atlas_handles: Query<(&Handle<TextureAtlas>, &TextureAtlasSprite)>,
    sprites: Query<AnyOf<(&Sprite, &TextureAtlasSprite)>>,
    mut despawn_particles_event_reader: EventReader<DespawnParticlesEvent>,
    no_death_animations: Query<&NoDespawnAnimation>,
    velocities: Query<&Velocity>,
) {
    for DespawnParticlesEvent { entity, linvel, angvel, phys_is_additive, lifetime } in despawn_particles_event_reader.iter() {
        if let Some(mut entity_commands) = commands.get_entity(*entity) {
            entity_commands.despawn();
            // Now spawn the death animation, if possible
            if no_death_animations.get(*entity).is_ok() {
                // We ignore death animations for this object.
                continue;
            }

            let (sheet_offset, texture_size, image_handle): (Vec2, Vec2, Handle<Image>) = if let Ok(image_handle) = image_handles.get(*entity) {
                if let Some(size) = images.get(&image_handle).and_then(|img| Some(img.size())) {
                    // Single image sprites will use an offset of 0,0
                    (Default::default(), size, image_handle.clone())
                }
                else {
                    warn!("Could not get image data to generate death particles for entity {:?}", entity);
                    continue;
                }

            }
            else if let Ok((atlas_handle, texture_atlas_sprite)) = atlas_handles.get(*entity) {

                if let Some(atlas) = atlases.get(&atlas_handle) {
                    if let Some(rect) = atlas.textures.get(texture_atlas_sprite.index) {
                        (rect.min, Vec2::new(rect.width(), rect.height()), atlas.texture.clone())
                    }
                    else {
                        error!("Invalid texture atlas index for sprite sheet when spawning death particles, entity {:?}", entity);
                        continue;
                    }
                }
                else {
                    warn!("Atlas not found when spawning death particles: entity {:?}", entity);
                    continue
                }
            }
            else {
                warn!("Generated a death particles event for an entity ({:?}) with no sprite", entity);
                continue;
            };

            let image_size = if let Some(size) = images.get(&image_handle).and_then(|img| Some(img.size())) {
                size
            }
            else {
                error!("Could not get image dimensions while geneating death particles for entity: {:?}", entity); 
                continue;
            };

            // See if there's a custom size set
            let custom_size = if let Ok((maybe_sprite, maybe_tas)) = sprites.get(*entity) {
                maybe_sprite.and_then(|sprite| sprite.custom_size).or(maybe_tas.and_then(|tas| tas.custom_size)) 
            }
            else {
                None
            };

            if let Ok(transform) = global_transforms.get(*entity) {
                let transform: Transform = (*transform).into();

                
                let real_size = custom_size.unwrap_or(texture_size);

                let section_size = (real_size * transform.scale.truncate()) / 8.0;
                let percent_size = (texture_size * transform.scale.truncate() / 8.0) / (image_size * transform.scale.truncate());
                let output_size = section_size;
                let center_point = transform.translation;

                let offsetter = (real_size * transform.scale.truncate()) / 2.0 - (0.5 * section_size);

                let num_rows = ((real_size.x * transform.scale.x) / section_size.x) as u32;
                let num_cols = ((real_size.y * transform.scale.y) / section_size.y) as u32;

                 

                for x in 0..num_rows {
                    for y in 0..num_cols {
                        let offset =
                            Vec2::new(section_size.x * x as f32, section_size.y * (num_cols - y - 1) as f32)
                                - offsetter;
                        let sheet_offset = sheet_offset / image_size;
                        let mesh =
                            Mesh2dHandle(meshes.add(shape::Quad::new(output_size).into()));
                        let translation = center_point 
                            + transform.rotation.normalize().mul_vec3(offset.extend(0.0));
                        let angle = angle_between3(center_point, translation)
                            + rand::thread_rng().gen_range(-0.8..0.8);
                        let velocity =
                            Vec2::new(DEATH_VEL * angle.sin(), DEATH_VEL * angle.cos());
                        let parent_velocity =
                            velocities.get(*entity).copied().unwrap_or_default();

                        // Use the parent's last known angvel to calculate additional linear
                        // velocity
                        let perp_angle = angle - (std::f32::consts::PI / 2.0);
                        let radius = center_point.distance(translation);
                        let total_velocity_from_angvel = radius * parent_velocity.angvel;
                        let additional_velocity_from_angvel = Vec2::new(total_velocity_from_angvel * perp_angle.sin(),
                                total_velocity_from_angvel * perp_angle.cos());

                        let total_linvel = if *phys_is_additive {
                            // Include the contextual velocities
                            velocity + parent_velocity.linvel + additional_velocity_from_angvel 
                        }

                        else {
                            // Ignores the above given velocities and just uses the generated one.
                            Vec2::ZERO
                        } + linvel.as_ref().and_then(|p| Some(p.get_value())).unwrap_or_default();

                        let material = despawn_materials.add(DespawnMaterial {
                            alpha: 1.0,
                            source_image: image_handle.clone(),
                            offset: (Vec2::new(x as f32, y as f32) * percent_size) + sheet_offset,
                            size: percent_size,
                        });
                        commands.spawn((
                            DespawnParticleBundle {
                                despawn_particle: DespawnParticle::new(lifetime.get_value()),
                                velocity: Velocity {
                                    linvel: total_linvel,
                                    angvel: angvel.as_ref().and_then(|p| Some(p.get_value())).unwrap_or_default(),
                                },
                                ..default()
                            },
                            mesh,
                            material,
                            SpatialBundle {
                                transform: Transform {
                                    translation,
                                    rotation: transform.rotation,
                                    ..default()
                                    //scale: transform.scale,
                                },
                                ..default()
                            },
                        ));
                    }
                }
            }
        }
    }
}

pub fn handle_despawn_particle(
    mut despawn_particles: Query<(Entity, &Handle<DespawnMaterial>, &mut DespawnParticle, &mut Transform)>,
    mut despawn_materials: ResMut<Assets<DespawnMaterial>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, material_handle, mut despawn_particle, mut transform) in despawn_particles.iter_mut() {
        despawn_particle.lifetime.tick(time.delta());
        if despawn_particle.lifetime.finished() {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
        let percent = despawn_particle.lifetime.percent_left();
        if let Some(mut material) = despawn_materials.get_mut(material_handle) {
            material.alpha = percent;
        }
        transform.scale = Vec3::splat(percent);
    }
}

