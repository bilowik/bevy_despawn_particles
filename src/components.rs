use bevy::prelude::*;

#[cfg(feature = "rapier")]
use bevy_rapier2d::prelude::*;

#[cfg(not(feature = "rapier"))]
use crate::phys::*;

/// A particle with an expiration
#[derive(Component)]
pub struct DespawnParticle {
    /// When this timer ends, the particle will despawn.
    pub lifetime: Timer,
}

impl DespawnParticle {
    /// Create a new despawn particle with the given lifetime
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

/// A despawn particle that will fade as it approaches its expiration
#[derive(Component, Default)]
pub(crate) struct FadingDespawnParticle;

/// A despawn particle that will shrink as it approaches its expiration
#[derive(Component, Default)]
pub(crate) struct ShrinkingDespawnParticle;

#[derive(Bundle)]
pub(crate) struct DespawnParticleBundle {
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
