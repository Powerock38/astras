#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings   globals

#import "shaders/atmosphere.wgsl" atmosphere

struct PlanetMaterial {
    color: vec4<f32>,
    seed: f32,
    noise_scale: f32,
    u: f32,
    atmosphere_scale: f32,
    atmosphere_density: f32,
    atmosphere_color: vec4<f32>,
};

@group(1) @binding(0) var<uniform> material: PlanetMaterial;

@fragment
fn fragment(
    in: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let len = length(in.uv - vec2f(0.5, 0.5));

    let atmo = atmosphere(in.uv, material.atmosphere_color.xyz, material.atmosphere_density);

    if (len < (0.5 - material.atmosphere_scale)) {
        return planet(in.uv) + atmo * 0.5;
    } else {
        return atmo;
    }
}

// Planet

fn hash23(p: vec2f) -> vec3f {
    let q = vec3f(dot(p, vec2f(127.1, 311.7)),
        dot(p, vec2f(269.5, 183.3)),
        dot(p, vec2f(419.2, 371.9)));
    return fract(sin(q) * 43758.5453);
}

fn voroNoise2(x: vec2f, u: f32, v: f32) -> f32 {
    let p = floor(x);
    let f = fract(x);
    let k = 1. + 63. * pow(1. - v, 4.);
    var va: f32 = 0.;
    var wt: f32 = 0.;
    for(var j: i32 = -2; j <= 2; j = j + 1) {
        for(var i: i32 = -2; i <= 2; i = i + 1) {
            let g = vec2f(f32(i), f32(j));
            let o = hash23(p + g) * vec3f(u, u, 1.);
            let r = g - f + o.xy;
            let d = dot(r, r);
            let ww = pow(1. - smoothstep(0., 1.414, sqrt(d)), k);
            va = va + o.z * ww;
            wt = wt + ww;
        }
    }
    return va / wt;
}

fn planet(
  uv: vec2<f32>,
) -> vec4<f32> {
    let noise = voroNoise2((uv + material.seed) * material.noise_scale, material.u, 0.0);
    var color = material.color.xyz * max(0.1, noise * 0.5);

    // Calculate the glow factor based on the brightness of the noise
    let glowThreshold = 0.7;
    let glowFactor = max(0.0, (noise - glowThreshold) / (1.0 - glowThreshold));

    let glowColor = material.color.xyz * 10.0;

    // Add the glow effect
    color += glowColor * glowFactor;

    return vec4f(color, 1.0);
}
