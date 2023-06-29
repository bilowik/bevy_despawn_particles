//! An event-based plugin for the Bevy game engine that provides a simple way to add a despawn
//! effect for 2D sprites.
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_despawn_particles::prelude::*;
//! 
//! #[derive(Component, Default)]
//! pub struct Marker;
//! 
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugin(DespawnParticlesPlugin)
//!         .add_system(setup.on_startup())
//!         .add_system(despawn)
//!         .run();
//! }
//! 
//! fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     commands.spawn(Camera2dBundle::default());
//!     commands
//!         .spawn(SpriteBundle {
//!             texture: asset_server.load("asteroid_round.png"),
//!             ..default()
//!         })
//!         .insert(Marker);
//! }
//! 
//! fn despawn(
//!     mut despawn_particles_event_writer: EventWriter<DespawnParticlesEvent>,
//!     entities: Query<Entity, Added<Marker>>,
//! ) {
//!     if let Ok(entity) = entities.get_single() {
//!         despawn_particles_event_writer.send(
//!             DespawnParticlesEvent::builder()
//!                 .with_fade(true)
//!                 .with_shrink(true)
//!                 .with_linvel(150.0)
//!                 .with_angvel([-5.0, -2.5, 2.5, 5.0])
//!                 .with_mass(1.0)
//!                 .with_lifetime(1.0)
//!                 .with_angular_damping(1.0)
//!                 .with_linear_damping(1.0)
//!                 .build(entity),
//!         );
//!     }
//! }
//! 
//!
//! ```
use bevy::app::{App, Plugin};
use bevy::asset::{load_internal_asset, AddAsset};
use bevy::ecs::prelude::IntoSystemConfig;
use bevy::ecs::schedule::SystemSet;
use bevy::render::prelude::Shader;

use bevy::sprite::Material2dPlugin;

#[cfg(feature = "rapier")]
use bevy_rapier2d::prelude::*;

pub mod components;
mod despawn;
pub mod events;
mod systems;

#[cfg(not(feature = "rapier"))]
mod phys;

mod utils;

use despawn::{DespawnMaterial, DESPAWN_MATERIAL_SHADER_HANDLE};
use events::DespawnParticlesEvent;
use systems::{handle_despawn_particle, handle_despawn_particles_event};

/// The despawn particle plugin. Required to utilize this crate.
#[derive(Default)]
pub struct DespawnParticlesPlugin;

/// The SystemSet that the flicker systems belong to.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct DespawnParticlesSet;

impl Plugin for DespawnParticlesPlugin {
    fn build(&self, app: &mut App) {
        // Register the flicker mateiral as an internal asset
        load_internal_asset!(
            app,
            DESPAWN_MATERIAL_SHADER_HANDLE,
            "despawn_material.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(Material2dPlugin::<DespawnMaterial>::default())
            .register_asset_reflect::<DespawnMaterial>();

        // Register events
        app.add_event::<DespawnParticlesEvent>();

        // Register systems and systemset
        // TODO: These might need to be ordered to prevent conflicts potentially?
        app.add_system(handle_despawn_particle.in_set(DespawnParticlesSet));
        app.add_system(handle_despawn_particles_event.in_set(DespawnParticlesSet));

        #[cfg(not(feature = "rapier"))]
        {
            app.add_system(phys::phys_tick.in_set(DespawnParticlesSet));
            app.init_resource::<phys::Gravity>();
        }

        #[cfg(feature = "rapier")]
        {
            app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        }
    }
}

pub mod prelude {
    pub use crate::components::DespawnParticle;
    pub use crate::events::{DespawnParticlesEvent, DespawnParticlesPreset};
    pub use crate::{DespawnParticlesPlugin, DespawnParticlesSet};
}
