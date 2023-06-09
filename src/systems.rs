use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Mesh2dHandle};

#[cfg(feature = "rapier")]
use bevy_rapier2d::prelude::*;

use bevy_variable_property::prelude::*;

#[cfg(not(feature = "rapier"))]
use crate::phys::{Damping, Velocity};

use crate::{
    components::*,
    despawn::{DespawnMaterial, NoDespawnAnimation},
    events::DespawnParticlesEvent,
    utils::angle_between3,
};

use rand::Rng;

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
    mut despawn_particles_event_reader: EventReader<DespawnParticlesEvent>,
    no_death_animations: Query<&NoDespawnAnimation>,
    velocities: Query<&Velocity>,
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
    } in despawn_particles_event_reader.iter()
    {
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

            // sheet_offset: M,N for texture atlas sprites depending on
            // the currently active sprite, always 0,0 for regular sprites
            //
            // texture_size: The size of the texture being read from. Same as image_size for
            // sprites, and the active sprite from the spritesheet for the texture
            // atlas sprite.
            //
            // image_size: The size of the entire source texture. output_size for meshes, and the entire
            // texture size for sprites and texture atlas sprites. 
            //
            // output_size: Set to custom_size if one is set for parent, otherwise image_size. 
            let (sheet_offset, texture_size, image_size, real_size, image_handle): (Vec2, Vec2, Vec2, Vec2, Handle<Image>) =
                if let Ok((sprite, image_handle)) = sprites.get(*entity) {
                    let image_size = if let Some(image) = images.get(&image_handle) {
                        image.size()
                    } else {
                        warn!(
                            "Could not get image data to generate death particles for entity {:?}",
                            entity
                        );
                        continue;
                    };
                    let real_size = sprite.custom_size.unwrap_or(image_size);

                    (Default::default(), image_size, image_size, real_size, image_handle.clone())

                } else if let Ok((tas, ta)) = tass.get(*entity)
                {
                    if let Some(atlas) = atlases.get(&ta) {
                        if let Some(rect) = atlas.textures.get(tas.index) {
                            if let Some(image) = images.get(&atlas.texture) {
                                let sheet_offset = rect.min;
                                let texture_size = Vec2::new(rect.width(), rect.height());
                                let real_size = tas.custom_size.unwrap_or(texture_size);
                                (
                                    sheet_offset,
                                    texture_size,
                                    image.size(),
                                    real_size,
                                    atlas.texture.clone(),
                                )
                            }
                            else {
                                error!("Atlas has invalid texture, entity {:?}", entity);
                                continue;
                            }
                        } else {
                            error!("Invalid texture atlas index for sprite sheet when spawning death particles, entity {:?}", entity);
                            continue;
                        }
                    } else {
                        warn!(
                            "Atlas not found when spawning death particles: entity {:?}",
                            entity
                        );
                        continue;
                    }
                } else {
                    warn!(
                        "Generated a death particles event for an entity ({:?}) with no sprite",
                        entity
                    );
                    continue;
                };


            if let Ok(transform) = global_transforms.get(*entity) {
                let transform: Transform = (*transform).into();

                let section_size = (real_size * transform.scale.truncate()) / 8.0;
                let percent_size = (texture_size * transform.scale.truncate() / 8.0)
                    / (image_size * transform.scale.truncate());
                let output_size = section_size;
                let center_point = transform.translation;

                let offsetter =
                    (real_size * transform.scale.truncate()) / 2.0 - (0.5 * section_size);

                let num_rows = ((real_size.x * transform.scale.x) / section_size.x) as u32;
                let num_cols = ((real_size.y * transform.scale.y) / section_size.y) as u32;

                for x in 0..num_rows {
                    for y in 0..num_cols {
                        let offset = Vec2::new(
                            section_size.x * x as f32,
                            section_size.y * (num_cols - y - 1) as f32,
                        ) - offsetter;
                        let sheet_offset = sheet_offset / image_size;
                        let mesh = Mesh2dHandle(meshes.add(shape::Quad::new(output_size).into()));
                        let translation = center_point
                            + transform.rotation.normalize().mul_vec3(offset.extend(0.0));
                        let angle = angle_between3(center_point, translation)
                            + rand::thread_rng().gen_range(-0.8..0.8);
                        let parent_velocity = velocities.get(*entity).copied().unwrap_or_default();

                        let vel_scalar = linvel.get_value();
                        let velocity =
                            Vec2::new(vel_scalar * angle.sin(), vel_scalar * angle.cos())
                                + if *ignore_parent_phys {
                                    Vec2::ZERO
                                } else {
                                    // Use the parent's last known angvel to calculate additional linear
                                    // velocity
                                    let perp_angle = angle - (std::f32::consts::PI / 2.0);
                                    let radius = center_point.distance(translation);
                                    let total_velocity_from_angvel =
                                        radius * parent_velocity.angvel;
                                    let additional_velocity_from_angvel = Vec2::new(
                                        total_velocity_from_angvel * perp_angle.sin(),
                                        total_velocity_from_angvel * perp_angle.cos(),
                                    );
                                    parent_velocity.linvel + additional_velocity_from_angvel
                                }
                                + linvel_addtl.get_value();

                        let material = despawn_materials.add(DespawnMaterial {
                            alpha: 1.0,
                            source_image: image_handle.clone(),
                            offset: (Vec2::new(x as f32, y as f32) * percent_size) + sheet_offset,
                            size: percent_size,
                        });
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
                            mesh,
                            material,
                            SpatialBundle {
                                transform: Transform {
                                    translation,
                                    rotation: transform.rotation,
                                    ..default() //scale: transform.scale,
                                },
                                ..default()
                            },
                        ));

                        shrink_spawn_func(&mut entity_cmds);
                        fade_spawn_func(&mut entity_cmds);
                    }
                }
            }
        }
    }
}

pub(crate) fn handle_despawn_particle(
    mut despawn_particles: Query<(
        Entity,
        &Handle<DespawnMaterial>,
        &mut DespawnParticle,
        &mut Transform,
        Option<&ShrinkingDespawnParticle>,
        Option<&FadingDespawnParticle>,
    )>,
    mut despawn_materials: ResMut<Assets<DespawnMaterial>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, material_handle, mut despawn_particle, mut transform, maybe_shrink, maybe_fade) in
        despawn_particles.iter_mut()
    {
        despawn_particle.lifetime.tick(time.delta());
        if despawn_particle.lifetime.finished() {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn();
            }
        }
        let percent = despawn_particle.lifetime.percent_left();
        if maybe_fade.is_some() {
            if let Some(material) = despawn_materials.get_mut(material_handle) {
                material.alpha = percent;
            }
        }
        if maybe_shrink.is_some() {
            transform.scale = Vec3::splat(percent);
        }
    }
}
