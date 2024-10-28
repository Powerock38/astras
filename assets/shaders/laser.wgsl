#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/noise.wgsl"::noise

struct LaserMaterial {
    color: vec4<f32>,
    seed: f32,
};

@group(2) @binding(0) var<uniform> material: LaserMaterial;


@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    var s = max(
        abs(sin((-in.uv.x + material.seed + globals.time) * 12.0) / 2.0),
        0.2
    );

    let n = noise(in.uv + material.seed + globals.delta_time);

    if in.uv.x < 0.2 {
        s *= in.uv.x * 1 / 0.2;
    }

    let p = max(s - abs(in.uv.y - 0.5), 0.0);

    var laser = p * n;

    if laser < 0.01 {
        discard;
    }

    return vec4<f32>(
        material.color.r * 2.0,
        material.color.g * 2.0,
        material.color.b * 2.0,
        laser,
    );
}
