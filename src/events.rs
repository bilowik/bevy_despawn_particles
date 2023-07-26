//! Event and related utilities for triggering despawn particles events
use bevy::prelude::*;
use bevy_variable_property::Property;

use crate::components::Curve;

impl DespawnParticlesPreset {
    /// Creates an event from the given preset.
    pub fn create_event(&self, entity: Entity) -> DespawnParticlesEvent {
        DespawnParticlesEvent {
            entity,
            angvel: self.angvel.clone(),
            linvel: self.linvel.clone(),
            linvel_addtl: self.linvel_addtl.clone(),
            linear_damping: self.linear_damping.clone(),
            angular_damping: self.angular_damping.clone(),
            mass: self.mass.clone(),
            lifetime: self.lifetime.clone(),
            ignore_parent_phys: self.ignore_parent_phys.clone(),
            shrink: self.shrink.clone(),
            fade: self.fade.clone(),
            mesh_override: self.mesh_override.clone(),
            target_num_particles: self.target_num_particles.clone(),
            gray: false,
        }
    }
}

/// Causes the given entity to be despawned and
/// [DespawnParticles][crate::components::DespawnParticle] to be generated.
///
/// Each of the given properties is applied to each of the generated
/// [DespawnParticles][crate::components::DespawnParticle].
#[derive(Clone)]
pub struct DespawnParticlesEvent {
    /// The target entity
    pub entity: Entity,

    /// The angular velocity
    pub angvel: Property<f32>,

    /// The linear velocity. The actual velocity vector is calculated using this and the angle the
    /// particle is from the center of the Entity.
    pub linvel: Property<f32>,

    /// Additive velocity that is applied uniformly to all generated particles. This does not take
    /// into account the angle the particle is from the center of the entity.
    pub linvel_addtl: Property<Vec2>,

    /// The friction factor for linear velocity. Non-zero values here will
    /// decelerate the particles. A negative value will accelerate them.
    pub linear_damping: Property<f32>,

    /// The friction factor for angular velocity. Non-zero values here will
    /// decelerate the particles. A negative value will accelerate them.
    pub angular_damping: Property<f32>,

    /// The length of time the generated particles will live for.
    pub lifetime: Property<f32>,

    /// The mass
    pub mass: Property<f32>,

    /// When true, the generated particles will ignore the target entity's velocities. When false, the
    /// target's velocity is added to each generated particle.
    pub ignore_parent_phys: bool,

    /// When true, generated particles will shrink as it's lifetime approaches 0.
    pub shrink: Option<Curve>,

    /// When true, generated particles will fade as it's lifetime approaches 0.
    pub fade: Option<Curve>,

    /// Use this mesh over the one used by the entity
    pub mesh_override: Option<Handle<Mesh>>,

    /// The number of particles to try to match. The actual number may be more than this.
    pub target_num_particles: Property<usize>,

    /// When true, will grayscale the particles
    pub gray: bool,
}

/// The builder struct for [DespawnParticlesEvent], typically this should be instantiated with
/// [DespawnParticlesEvent::builder].
#[derive(Clone)]
pub struct DespawnParticlesEventBuilder {
    pub angvel: Property<f32>,
    pub linvel: Property<f32>,
    pub linvel_addtl: Property<Vec2>,
    pub linear_damping: Property<f32>,
    pub angular_damping: Property<f32>,
    pub lifetime: Property<f32>,
    pub mass: Property<f32>,
    pub ignore_parent_phys: bool,
    pub shrink: Option<Curve>,
    pub fade: Option<Curve>,
    pub mesh_override: Option<Handle<Mesh>>,
    pub target_num_particles: Property<usize>,
    pub gray: bool,
}

impl DespawnParticlesEvent {
    pub fn builder() -> DespawnParticlesEventBuilder {
        DespawnParticlesEventBuilder::new()
    }
}

impl DespawnParticlesEventBuilder {
    pub fn new() -> Self {
        Self {
            angvel: Default::default(),
            linvel: Default::default(),
            linvel_addtl: Default::default(),
            lifetime: 1.0.into(),
            linear_damping: Default::default(),
            angular_damping: Default::default(),
            mass: Default::default(),
            ignore_parent_phys: false,
            shrink: None,
            fade: None,
            mesh_override: None,
            target_num_particles: 64.into(),
            gray: false,
        }
    }

    /// See [DespawnParticlesEvent::angvel]
    pub fn with_angvel<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.angvel = v.into();
        self
    }

    /// See [DespawnParticlesEvent::linvel]
    pub fn with_linvel<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.linvel = v.into();
        self
    }

    /// See [DespawnParticlesEvent::lifetime]
    pub fn with_lifetime<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.lifetime = v.into();
        self
    }

    /// See [DespawnParticlesEvent::ignore_parent_phys]
    pub fn with_ignore_parent_phys(mut self, ignore_parent_phys: bool) -> Self {
        self.ignore_parent_phys = ignore_parent_phys;
        self
    }

    /// See [DespawnParticlesEvent::linvel_addtl]
    pub fn with_linvel_addtl<T: Into<Property<Vec2>>>(mut self, v: T) -> Self {
        self.linvel_addtl = v.into();
        self
    }

    /// See [DespawnParticlesEvent::linear_damping]
    pub fn with_linear_damping<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.linear_damping = v.into();
        self
    }

    /// See [DespawnParticlesEvent::angular_damping]
    pub fn with_angular_damping<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.angular_damping = v.into();
        self
    }

    /// See [DespawnParticlesEvent::mass]
    pub fn with_mass<T: Into<Property<f32>>>(mut self, v: T) -> Self {
        self.mass = v.into();
        self
    }

    /// See [DespawnParticlesEvent::shrink]
    pub fn with_shrink(mut self, shrink: Option<Curve>) -> Self {
        self.shrink = shrink;
        self
    }

    /// See [DespawnParticlesEvent::fade]
    pub fn with_fade(mut self, fade: Option<Curve>) -> Self {
        self.fade = fade;
        self
    }

    pub fn with_mesh_override(mut self, mesh_override: Handle<Mesh>) -> Self {
        self.mesh_override = Some(mesh_override);
        self
    }
    /// See [DespawnParticlesEvent::target_num_particles]
    pub fn with_target_num_particles<T: Into<Property<usize>>>(mut self, v: T) -> Self {
        self.target_num_particles = v.into();
        self
    }

    /// See [DespawnParticlesEvent::gray]
    pub fn with_gray(mut self, gray: bool) -> Self {
        self.gray = gray;
        self
    }

    pub fn build(self, entity: Entity) -> DespawnParticlesEvent {
        DespawnParticlesEvent {
            entity,
            angvel: self.angvel,
            linvel: self.linvel,
            linvel_addtl: self.linvel_addtl,
            linear_damping: self.linear_damping,
            angular_damping: self.angular_damping,
            mass: self.mass,
            lifetime: self.lifetime,
            ignore_parent_phys: self.ignore_parent_phys,
            shrink: self.shrink,
            fade: self.fade,
            mesh_override: self.mesh_override,
            target_num_particles: self.target_num_particles,
            gray: self.gray,
        }
    }
}

/// Defines a preset for [DespawnParticlesEvent] that can be used to repeatedly generate
/// events with the same parameters using [DespawnParticlesPreset::create_event]
pub type DespawnParticlesPreset = DespawnParticlesEventBuilder;
