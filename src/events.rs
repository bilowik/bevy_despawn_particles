use bevy::prelude::*;
use bevy_variable_property::Property;

#[derive(Clone)]
pub struct DespawnParticlesEvent {
    pub entity: Entity,
    pub angvel: Option<Property<f32>>,
    pub linvel: Option<Property<Vec2>>,
    pub phys_is_additive: bool,
}

#[derive(Clone)]
pub struct DespawnParticlesEventBuilder {
    pub entity: Entity,
    pub angvel: Option<Property<f32>>,
    pub linvel: Option<Property<Vec2>>,
    pub phys_is_additive: bool,
}

impl DespawnParticlesEvent {

    pub fn builder(entity: Entity) -> DespawnParticlesEventBuilder {
        DespawnParticlesEventBuilder::new(entity)
    }
}

impl DespawnParticlesEventBuilder {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            angvel: None,
            linvel: None,
            phys_is_additive: true,
        }
    }

    pub fn with_angvel<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.angvel = Some(v.into());
        self
    }

    pub fn with_linvel<T: Into<Property<Vec2>>>(mut self, v: T) -> Self {
        self.linvel = Some(v.into());
        self
    }
    
    pub fn with_additive_phys(mut self, phys_is_additive: bool) -> Self {
        self.phys_is_additive = phys_is_additive;
        self
    }

    pub fn build(self) -> DespawnParticlesEvent {
        DespawnParticlesEvent {
            entity: self.entity,
            angvel: self.angvel,
            linvel: self.linvel,
            phys_is_additive: self.phys_is_additive,
        }
    }
}
