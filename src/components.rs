use bevy::{
    prelude::*,
    render::{
        render_resource::PrimitiveTopology,
        mesh::Indices,
    }
};

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

/// Used for ColorMaterial meshes to track what the original alpha value
/// was so it can be properly mixed during fading.
#[derive(Component)]
pub struct OriginalAlpha(pub f32);

impl Default for OriginalAlpha {
    fn default() -> Self {
        Self(1.0)
    }
}

/// When present on an Entity, will override the underlying Mesh when creating the
/// despawn particles. Targetted mostly towards circles since the way they are built do 
/// not break down in a way similar to other shapes. 
#[derive(Component, Reflect, FromReflect, Default)]
#[reflect(Component)]
pub struct DespawnMeshOverride(pub Handle<Mesh>);

impl DespawnMeshOverride {
    /// Creates a polygon inscribed in circle with the given radius, with the indices and vertices
    /// set up in a way to break apart in a cleaner way. 
    /// 
    /// For a circle, 9-13 sides is sufficient, going higher will yield more sliver particles.
    ///
    /// See the circle's in examples/mesh.rs for a visualization of the difference.
    ///
    pub fn faux_circle(meshes: &mut Assets<Mesh>, radius: f32, sides: u32) -> Self {
        let vertices = std::iter::once([0.0, 0.0, 0.0])
            .chain(
                (0..sides)
                    .map(|v| (v as f32) * (2.0 / sides as f32) * std::f32::consts::PI)
                    .map(|angle: f32| [angle.cos() * radius, angle.sin() * radius, 0.0]),
            )
            .collect::<Vec<_>>();

        let indices = (0..(sides as u32 - 1))
            .map(|idx| [0, idx + 1, idx + 2])
            .chain(std::iter::once([0, sides, 1]))
            .flatten()
            .collect::<Vec<_>>();
        let normals = (0..sides + 1).map(|_| [0.0, 0.0, 1.0]).collect::<Vec<_>>();
       

        // Calculate UVs by creating a box around this shape and calculating the percent offsets.
        let diameter = radius * 2.0;
        let uvs = vertices
            .iter()
            .map(|[x, y, _]| [(x + radius) / diameter, (y - radius) / (-diameter)])
            .collect::<Vec<_>>();
        
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));

        Self(meshes.add(mesh))
    }
}
