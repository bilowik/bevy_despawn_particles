use bevy::prelude::Entity;

#[derive(Debug, Clone)]
pub struct DespawnParticlesEvent {
    pub entity: Entity,
}
