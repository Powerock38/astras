#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/noise.wgsl"::nestedMovingNoise

const NOISE_SPEED: f32 = 0.04;

struct StarMaterial {
    color: vec4<f32>,
    seed: f32,
    rotation: vec2<f32>,
};

@group(2) @binding(0) var<uniform> material: StarMaterial;

const NOISE_SCALE: f32 = 8.0;
const GLOW_THRESHOLD: f32 = 0.4;
const GLOW_MULTIPLIER: f32 = 20.0;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv_rotation = sin(in.uv + material.rotation * globals.time) / 2.0 + 0.5;

    var color = material.color.xyz * nestedMovingNoise(uv_rotation * NOISE_SCALE, NOISE_SPEED, material.seed);

    let glowFactor = max(0.0, (max(color.x, max(color.y, color.z)) - GLOW_THRESHOLD) / (1.0 - GLOW_THRESHOLD));
    let glowColor = material.color.xyz * GLOW_MULTIPLIER;
    color += glowColor * glowFactor;

    return vec4f(color, 1.0);
}
