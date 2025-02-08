use bevy_asset::{Asset, Handle};
use bevy_image::Image;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_render::render_resource::{AsBindGroup, ShaderRef};
use bevy_sprite::Material2d;

// Needed for AsBindGroup derive macro since it expects the bevy crate.
mod bevy {
    pub mod render {
        pub use bevy_render::*;
    }
}

#[derive(AsBindGroup, Clone, Reflect, Asset)]
pub struct DespawnMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Option<Handle<Image>>,

    /// Percentage, as a value between 0.0 and 1.0
    #[uniform(2)]
    pub offset: Vec2,

    /// Percentage, as a value between 0.0 and 1.0
    #[uniform(2)]
    pub size: Vec2,

    /// Percentage, as a value between 0.0 and 1.0
    #[uniform(2)]
    pub alpha: f32,

    /// When true, grayscale the underlying texture
    #[uniform(2)]
    pub gray: u32,

    /// ensures 16-byte alignment.
    #[uniform(2)]
    pub padding: u32,
}

impl Material2d for DespawnMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://despawn_material.wgsl".into()
    }
}
