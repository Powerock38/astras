#import bevy_pbr::forward_io::VertexOutput
#import "shaders/noise.wgsl"::{randvec2f, nestedNoise}

struct BackgroundMaterial {
  seed: f32,
};

@group(2) @binding(0) var<uniform> material: BackgroundMaterial;

const NOISE_SCALE: f32 = 0.01;

const STAR_COLOR: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
const GALAXY_COLOR1: vec3<f32> = vec3<f32>(0.2, 0.1, 0.7);
const GALAXY_COLOR2: vec3<f32> = vec3<f32>(0.1, 0.2, 0.7);
const GALAXY_COLOR3: vec3<f32> = vec3<f32>(0.1, 0.1, 0.7);

const ENABLE_GALAXY: bool = true; // impact performance a lot

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

    if dots(in.position.xy * 0.5) < 0.1 {
        let c = randvec2f(in.position.xy + material.seed);
        return vec4f(STAR_COLOR, 1.0);
    }

    if !ENABLE_GALAXY {
        discard;
    }

    var color = vec3<f32>(0.0, 0.0, 0.0);

    let n1 = nestedNoise(in.position.xy * NOISE_SCALE, material.seed);
    if n1 > 0.5 {
        color += n1 * GALAXY_COLOR1 * 0.01;
    }

    let n2 = nestedNoise(in.position.xy * NOISE_SCALE * 0.5, material.seed);
    if n2 > 0.7 {
        color += n2 * GALAXY_COLOR2 * 0.01;
    }

    let n3 = nestedNoise(in.position.xy * NOISE_SCALE * 0.25, material.seed);
    if n3 > 0.8 {
        color += n3 * GALAXY_COLOR3 * 0.01;
    }

    if length(color) > 0.0 {
        return vec4f(color, 1.0);
    }

    discard;
}

fn dots(uv: vec2f) -> f32 {
    let g = floor(uv);
    let f = fract(uv);
    let r = randvec2f(g) * 0.19;
    return length(f + r);
}
