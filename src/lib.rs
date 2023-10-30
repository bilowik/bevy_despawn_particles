#![doc = include_str!("../README.md")]
use bevy_app::{App, Plugin, Startup, Update};
use bevy_asset::{load_internal_asset, AddAsset};
use bevy_ecs::schedule::{IntoSystemConfigs, SystemSet};
use bevy_render::render_resource::Shader;

use bevy_sprite::Material2dPlugin;

#[cfg(feature = "rapier")]
use bevy_rapier2d::prelude::*;

pub mod components;
mod despawn;
pub mod events;
pub mod resources;
mod systems;

#[cfg(not(feature = "rapier"))]
pub mod phys;

mod utils;

use despawn::{DespawnMaterial, DESPAWN_MATERIAL_SHADER_HANDLE};
use events::DespawnParticlesEvent; use resources::{DespawnParticleQueue, DespawnParticlesConfig}; use systems::{
    handle_despawn_particle, handle_despawn_particles_events, max_particles_check, setup,
};

/// The despawn particle plugin. Required to utilize this crate.
#[derive(Default)]
pub struct DespawnParticlesPlugin;

/// The SystemSet that the despawn particle systems belong to.
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

        app.add_plugins(Material2dPlugin::<DespawnMaterial>::default())
            .register_asset_reflect::<DespawnMaterial>();

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

        #[cfg(not(feature = "rapier"))]
        {
            app.add_systems(Update, phys::phys_tick.in_set(DespawnParticlesSet));
            app.init_resource::<phys::Gravity>();
        }

        #[cfg(feature = "rapier")]
        {
            app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
        }
    }
}

pub mod prelude {
    pub use crate::components::{DespawnMeshOverride, DespawnParticle, DespawnImageOverride};
    pub use crate::events::{DespawnParticlesEvent, DespawnParticlesPreset};
    pub use crate::resources::DespawnParticlesConfig;
    pub use crate::{DespawnParticlesPlugin, DespawnParticlesSet};
}
