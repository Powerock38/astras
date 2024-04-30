#import bevy_pbr::forward_io::VertexOutput
#import "shaders/noise.wgsl"::randvec2f

struct BackgroundMaterial {
  seed: f32,
};

@group(2) @binding(0) var<uniform> material: BackgroundMaterial;

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {

  if (dots(in.position.xy * 0.5) < 0.1) {
    let c = randvec2f(in.position.xy + material.seed);
    return vec4f(c.x, c.y, 0.0, 1.0);
  }

  discard;
}

fn dots(uv: vec2f) -> f32 {
    let g = floor(uv);
    let f = fract(uv);
    let r = randvec2f(g) * 0.19;
    return length(f+r);
}
