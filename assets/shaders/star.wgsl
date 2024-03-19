#import bevy_pbr::forward_io::VertexOutput
#import "shaders/noise.wgsl"::nestedNoise

const NOISE_SPEED: f32 = 0.04;

struct StarMaterial {
    color: vec4<f32>,
    seed: f32,
};

@group(2) @binding(0) var<uniform> material: StarMaterial;

const NOISE_SCALE: f32 = 8.0;
const GLOW_THRESHOLD: f32 = 0.4;
const GLOW_MULTIPLIER: f32 = 20.0;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    var color = material.color.xyz * nestedNoise(in.uv * NOISE_SCALE, NOISE_SPEED, material.seed);

    let glowFactor = max(0.0, (max(color.x, max(color.y, color.z)) - GLOW_THRESHOLD) / (1.0 - GLOW_THRESHOLD));
    let glowColor = material.color.xyz * GLOW_MULTIPLIER;
    color += glowColor * glowFactor;

    return vec4f(color, 1.0);
}
