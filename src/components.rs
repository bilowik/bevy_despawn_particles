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

#[derive(Bundle)]
pub struct DespawnParticleBundle {
    pub despawn_particle: DespawnParticle,
    pub mass: AdditionalMassProperties,
    pub velocity: Velocity,
    pub damping: Damping,
    #[cfg(feature = "rapier")]
    pub rigid_body: RigidBody,
}

impl Default for DespawnParticleBundle {
    fn default() -> Self {
        Self {
            despawn_particle: Default::default(),
            #[cfg(not(feature = "rapier"))]
            mass: 1.0.into(),
            #[cfg(feature = "rapier")]
            mass: AdditionalMassProperties::Mass(500.0),
            velocity: Default::default(),
            damping: Default::default(),
            #[cfg(feature = "rapier")]
            rigid_body: RigidBody::Dynamic,
        }
    }
}
