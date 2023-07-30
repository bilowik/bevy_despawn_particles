use std::collections::VecDeque;

use bevy::ecs::prelude::{Resource, Entity};

#[derive(Resource)]
pub struct DespawnParticlesConfig {
    pub max_particles: usize,
}

impl Default for DespawnParticlesConfig {
    fn default() -> Self {
        Self {
            max_particles: 1024,
        }
    }
}


#[derive(Resource, Default)]
pub struct DespawnParticleQueue(pub VecDeque<Entity>);
