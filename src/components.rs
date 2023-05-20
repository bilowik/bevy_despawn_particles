use bevy::prelude::*;

#[cfg(feature = "rapier")]
use bevy_rapier2d::prelude::*;

#[cfg(not(feature = "rapier"))]
use crate::phys::*;

#[derive(Component)]
pub struct DespawnParticle {
    pub lifetime: Timer,
}

impl DespawnParticle {
    pub fn new(seconds: f32) -> Self {
        Self {
            lifetime: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}

impl Default for DespawnParticle {
    fn default() -> Self {
        Self::new(1.0)
    }
}


// These are held separately for future implementation of 
// shrink/fade curves. For now it is strictly linear.
#[derive(Component, Default)]
pub struct FadingDespawnParticle;

#[derive(Component, Default)]
pub struct ShrinkingDespawnParticle;

#[cfg(feature = "rapier")]
#[derive(Bundle)]
pub struct DespawnParticleBundle {
    pub despawn_particle: DespawnParticle,
    pub mass_properties: AdditionalMassProperties,
    pub velocity: Velocity,
    pub acceleration: ExternalForce,
    pub rigid_body: RigidBody,
}

#[cfg(feature = "rapier")]
impl Default for DespawnParticleBundle {
    fn default() -> Self {
        Self {
            despawn_particle: Default::default(),
            mass_properties: AdditionalMassProperties::Mass(1.0),
            velocity: Default::default(),
            acceleration: Default::default(),
            rigid_body: Default::default(),
        }
    }
}


#[cfg(not(feature = "rapier"))]
#[derive(Bundle, Default)]
pub struct DespawnParticleBundle {
    pub despawn_particle: DespawnParticle,
    pub velocity: Velocity,
}

