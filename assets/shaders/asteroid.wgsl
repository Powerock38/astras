#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/noise.wgsl"::{nestedNoise, randvec2f, fractalNoise}

struct AsteroidMaterial {
    color: vec4<f32>,
    seed: f32,
};

@group(2) @binding(0) var<uniform> material: AsteroidMaterial;

const NOISE_SCALE: f32 = 10.0;
const CRATER_RADIUS: f32 = 0.3;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    var color = material.color.xyz * nestedNoise(in.uv * NOISE_SCALE, material.seed);

    // craters
    for (var i = 0; i < 10; i += 1) {
        let p = abs(randvec2f(vec2f(material.seed + f32(i)))) + fractalNoise(in.uv * NOISE_SCALE + material.seed + f32(i));
        if length(in.uv - p) - CRATER_RADIUS < 0.0 {
            color *= 0.3;
        break;
        }
    }

    return vec4f(color, 1.0);
}
