#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils
#import bevy_sprite::mesh2d_vertex_output::VertexOutput


struct DespawnMaterial {
    offset: vec2<f32>,
    size: vec2<f32>,
    alpha: f32,
    gray: u32,
    padding: u32
};

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> despawn_material: DespawnMaterial;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let old_range = 1.0;
    let new_range = despawn_material.size;
    let uv = ((in.uv * new_range) + despawn_material.offset);
    let color = textureSample(texture, our_sampler, uv);
    let value = (color.r * 0.299 + color.g * 0.587 + color.b * 0.114);
    var new_color: vec4<f32>;

    if(despawn_material.gray == 1u) {
    	new_color = vec4<f32>(value, value, value, 0.0);
    }
    else {
	new_color = color;
    }

    new_color[3] = despawn_material.alpha * color.a;
    return new_color;

}
