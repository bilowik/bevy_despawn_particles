use bevy::ecs::prelude::Resource;

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
