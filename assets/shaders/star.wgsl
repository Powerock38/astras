#import bevy_pbr::forward_io::VertexOutput
#import "shaders/noise.wgsl"::nestedNoise

const NOISE_SPEED: f32 = 0.04;

struct StarMaterial {
    color: vec4<f32>,
};

@group(2) @binding(0) var<uniform> material: StarMaterial;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return vec4f(material.color.xyz * nestedNoise(in.uv, NOISE_SPEED), 1.0);
}
