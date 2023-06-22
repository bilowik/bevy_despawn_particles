use bevy::prelude::*;
use bevy_variable_property::Property;

#[derive(Clone)]
pub struct DespawnParticlesEvent {
    pub entity: Entity,
    pub angvel: Property<f32>,
    pub linvel: Property<f32>,
    pub linvel_addtl: Property<Vec2>,
    pub linear_damping: Property<f32>,
    pub angular_damping: Property<f32>,
    pub lifetime: Property<f32>,
    pub mass: Property<f32>,
    pub ignore_parent_phys: bool,
}

#[derive(Clone)]
pub struct DespawnParticlesEventBuilder {
    pub entity: Entity,
    pub angvel: Property<f32>,
    pub linvel: Property<f32>,
    pub linvel_addtl: Property<Vec2>,
    pub linear_damping: Property<f32>,
    pub angular_damping: Property<f32>,
    pub lifetime: Property<f32>,
    pub mass: Property<f32>,
    pub ignore_parent_phys: bool,
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
            angvel: Default::default(),
            linvel: Default::default(),
            linvel_addtl: Default::default(),
            lifetime: 1.0.into(),
            linear_damping: Default::default(),
            angular_damping: Default::default(),
            mass: Default::default(),
            ignore_parent_phys: true,
        }
    }

    pub fn with_angvel<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.angvel = v.into();
        self
    }

    pub fn with_linvel<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.linvel = v.into();
        self
    }
    pub fn with_lifetime<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.lifetime = v.into();
        self
    }
    
    pub fn with_ignore_parent_phys(mut self, ignore_parent_phys: bool) -> Self {
        self.ignore_parent_phys = ignore_parent_phys;
        self
    }

    pub fn with_linvel_addtl<T: Into<Property<Vec2>>>(mut self, v: T) -> Self {
        self.linvel_addtl = v.into();
        self
    }

    pub fn with_linear_damping<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.linear_damping = v.into();
        self
    }

    pub fn with_angular_damping<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.angular_damping = v.into();
        self
    }

    pub fn with_mass<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.mass = v.into();
        self
    }
    pub fn build(self) -> DespawnParticlesEvent {
        DespawnParticlesEvent {
            entity: self.entity,
            angvel: self.angvel,
            linvel: self.linvel,
            linvel_addtl: self.linvel_addtl,
            linear_damping: self.linear_damping,
            angular_damping: self.angular_damping,
            mass: self.mass,
            lifetime: self.lifetime,
            ignore_parent_phys: self.ignore_parent_phys,
        }
    }
}
