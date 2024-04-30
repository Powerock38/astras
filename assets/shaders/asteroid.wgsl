#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/noise.wgsl"::{nestedNoise, randvec2f}

struct AsteroidMaterial {
    color: vec4<f32>,
    seed: f32,
};

@group(2) @binding(0) var<uniform> material: AsteroidMaterial;

const NOISE_SCALE: f32 = 8.0;
const CRATER_RADIUS: f32 = 1.0;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    var color = material.color.xyz * nestedNoise(in.uv * NOISE_SCALE, material.seed);

    // craters
    let p = randvec2f(in.uv + material.seed);
    if length(p) - CRATER_RADIUS < 0.0 {
        color *= 0.5;
    }

    return vec4f(color, 1.0);
}
