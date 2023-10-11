#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings   globals

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

    if (len < (0.5 - material.atmosphere_scale)) {
        return planet(in.uv) + atmosphere(in.uv) * 0.5;
    } else {
        return atmosphere(in.uv);
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



// Atmopshere

const add: vec2<f32> = vec2<f32>(1.0, 0.0);
const addz: vec2<f32> = vec2<f32>(0.0, 1.0);
const HASHSCALE1: f32 = 0.1031;
const SPEED: f32 = 100.0;

fn hash12(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3(p.xyx) * HASHSCALE1);
    p3 = p3 + dot(p3, p3.yzx + 19.19);
    return fract((p3.x + p3.y) * p3.z);
}

fn noise(x: vec2<f32>) -> f32 {
    let p = floor(x);
    var f = fract(x);
    f = f * f * (3.0 - 2.0 * f);
    let res = mix(
        mix(hash12(p), hash12(p + add.xy), f.x),
        mix(hash12(p + add.yx), hash12(p + add.xx), f.x),
        f.y
    );
    return res;
}

fn fractalNoise(xy: vec2<f32>, iTime: f32) -> f32 {
    var xy = xy;
    var w: f32 = 0.7;
    var f: f32 = 0.0;

    for (var i: i32 = 0; i < 4; i = i + 1) {
        f = f + noise(xy + vec2<f32>(f32(i) * iTime * 0.015 * SPEED * w)) * w;
        w = w * 0.5;
        xy = xy * 2.7;
    }

    return f;
}

fn waterMap(pos: vec2<f32>) -> f32 {
    let posm = pos * mat2x2(
        0.60, -0.80,
        0.80, 0.60
    );
    return abs(fractalNoise(vec2<f32>(8.0 * posm), 0.0) - 0.5) * 0.1;
}

fn acesTonemap(color: vec3<f32>) -> vec3<f32> {
    let m1 = mat3x3(
        0.59719, 0.07600, 0.02840,
        0.35458, 0.90834, 0.13383,
        0.04823, 0.01566, 0.83777
    );
    let m2 = mat3x3(
        1.60475, -0.10208, -0.00327,
        -0.53108, 1.10813, -0.07276,
        -0.07367, -0.00605, 1.07602
    );
    let v = m1 * color;
    let a = v * (v + vec3<f32>(0.0245786)) - vec3<f32>(0.000090537);
    let b = v * (vec3<f32>(0.983729) * v + vec3<f32>(0.4329510)) + vec3<f32>(0.238081);
    return pow(clamp(m2 * (a / b), vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(1.0 / 2.2));
}

fn atmosphere(uv: vec2f) -> vec4f {
    var uv = uv;
    var normal: vec2<f32>;
    normal.x = (waterMap(uv + add) - waterMap(uv - add)) / (2.0 * 0.1);
    normal.y = (waterMap(uv + addz) - waterMap(uv - addz)) / (2.0 * 0.1);
    var water: f32 = waterMap(normal);
    var col = material.atmosphere_color.xyz;
    col = col + water;
    uv = uv * 5.0;
    var noise1: f32 = (fractalNoise(uv, globals.time) - 0.55) * 5.0;
    var noise2: f32 = (fractalNoise(uv + vec2<f32>(0.25, 0.25), globals.time) - 0.55) * 5.0;
    col = mix(col, acesTonemap(vec3<f32>(0.65, 0.65, 0.75)), clamp((noise2 * 0.1 - 0.1) / water, 0.0, 1.0) * 0.1);
    col = mix(col, acesTonemap(vec3<f32>(1.0, 1.0, 1.09)), clamp((noise1 * 0.1 - 0.1) / water, 0.0, 1.0) * 0.1);

    return vec4f(col, material.atmosphere_density);
}