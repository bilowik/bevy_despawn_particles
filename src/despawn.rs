use bevy_reflect::TypeUuid;
use bevy_render::render_resource::{AsBindGroup, ShaderRef, Shader};
use bevy_sprite::Material2d;
use bevy_ecs::{
    component::Component,
    reflect::ReflectComponent,
};
use bevy_reflect::Reflect;
use bevy_math::Vec2;
use bevy_asset::{Handle, HandleUntyped};
use bevy_render::texture::Image;

pub const DESPAWN_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13255228607086843049);

// Needed for AsBindGroup derive macro since it expects the bevy crate.
mod bevy {
    pub mod render {
        pub use bevy_render::*;
    }
}


#[derive(AsBindGroup, TypeUuid, Clone, Reflect)]
#[uuid = "f3bd99b1-6bd7-4749-97ae-0b526d1b6aed"]
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
}

impl Material2d for DespawnMaterial {
    fn fragment_shader() -> ShaderRef {
        DESPAWN_MATERIAL_SHADER_HANDLE.typed().into()
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct NoDespawnAnimation;
