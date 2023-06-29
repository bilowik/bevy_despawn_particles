use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

pub const DESPAWN_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 13255228607086843049);

#[derive(AsBindGroup, TypeUuid, Clone, Reflect, FromReflect)]
#[uuid = "f3bd99b1-6bd7-4749-97ae-0b526d1b6aed"]
pub struct DespawnMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,

    /// Percentage, as a value between 0.0 and 1.0
    #[uniform(2)]
    pub offset: Vec2,

    /// Percentage, as a value between 0.0 and 1.0
    #[uniform(2)]
    pub size: Vec2,

    /// Percentage, as a value between 0.0 and 1.0
    #[uniform(2)]
    pub alpha: f32,
}

impl Material2d for DespawnMaterial {
    fn fragment_shader() -> ShaderRef {
        DESPAWN_MATERIAL_SHADER_HANDLE.typed().into()
    }
}

#[derive(Component, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct NoDespawnAnimation;
