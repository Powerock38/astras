#import bevy_sprite::mesh2d_vertex_output MeshVertexOutput

struct BackgroundMaterial {
  seed: f32,
};

@fragment
fn fragment(
    in: MeshVertexOutput
) -> @location(0) vec4<f32> {

  if (dots(in.position.xy * 0.5) < 0.1) {
    let c = randvec2f(in.position.xy);
    return vec4f(c.x, c.y, 0.0, 1.0);
  }

  return vec4f(0.0);
}

fn dots(uv: vec2f) -> f32 {
    let g = floor(uv);
    let f = fract(uv);
    let r = randvec2f(g) * 0.19;
    return length(f+r);
}

fn randvec2f(co: vec2f) -> vec2f {
    return vec2(
        fract(sin(dot(co.xy, vec2(12.9898,78.233))) * 43758.5453),
        fract(cos(dot(co.yx,vec2(8.64947,45.097))) * 43758.5453)
    ) * 2.0 - 1.0;
}
