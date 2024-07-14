#![doc = include_str!("../README.md")]
use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::schedule::{IntoSystemConfigs, SystemSet};

use bevy_sprite::Material2dPlugin;

#[cfg(feature = "bevy_rapier2d")]
use bevy_rapier2d::prelude::*;

pub mod components;
mod despawn;
pub mod events;
pub mod resources;
mod systems;

#[cfg(not(feature = "bevy_rapier2d"))]
pub mod phys;

mod utils;

use despawn::DespawnMaterial;
use events::DespawnParticlesEvent;
use resources::{DespawnParticleQueue, DespawnParticlesConfig};
use systems::{
    handle_despawn_particle, handle_despawn_particles_events, max_particles_check, setup,
};

use std::path::{Path, PathBuf};

/// The despawn particle plugin. Required to utilize this crate.
#[derive(Default)]
pub struct DespawnParticlesPlugin;

/// The SystemSet that the despawn particle systems belong to.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct DespawnParticlesSet;

impl Plugin for DespawnParticlesPlugin {
    fn build(&self, app: &mut App) {
        // Register the flicker mateiral as an internal asset
        let embedded = app
            .world_mut()
            .resource_mut::<::bevy_asset::io::embedded::EmbeddedAssetRegistry>();
        let path = Path::new("despawn_material.wgsl");
        embedded.insert_asset(
            PathBuf::new(),
            &path,
            include_bytes!("despawn_material.wgsl"),
        );

        app.add_plugins(Material2dPlugin::<DespawnMaterial>::default())
            .register_type::<DespawnMaterial>();

        // Register events
        app.add_event::<DespawnParticlesEvent>();

        // Register systems and systemset
        // TODO: These might need to be ordered to prevent conflicts potentially?
        app.add_systems(Update, handle_despawn_particle.in_set(DespawnParticlesSet));
        app.add_systems(
            Update,
            handle_despawn_particles_events.in_set(DespawnParticlesSet),
        );
        app.add_systems(Update, max_particles_check.in_set(DespawnParticlesSet));
        app.add_systems(Startup, setup);

        app.init_resource::<DespawnParticlesConfig>();
        app.init_resource::<DespawnParticleQueue>();

        #[cfg(not(feature = "bevy_rapier2d"))]
        {
            app.add_systems(Update, phys::phys_tick.in_set(DespawnParticlesSet));
            app.init_resource::<phys::Gravity>();
        }

        #[cfg(feature = "bevy_rapier2d")]
        {
            app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        }
    }
}

pub mod prelude {
    pub use crate::components::{DespawnMeshOverride, DespawnParticle};
    pub use crate::events::{DespawnParticlesEvent, DespawnParticlesPreset};
    pub use crate::resources::DespawnParticlesConfig;
    pub use crate::{DespawnParticlesPlugin, DespawnParticlesSet};
}
