#import bevy_pbr::forward_io::VertexOutput
#import "shaders/noise.wgsl"::{random, nestedNoise, nestedMovingNoise}

const NB_COLORS = 3u;

struct PlanetMaterial {
    colors: array<vec4<f32>, NB_COLORS>,
    seed: f32,
    noise_scale: f32,
    planet_radius_normalized: f32,
    atmosphere_density: f32,
    atmosphere_color: vec4<f32>,
    atmosphere_speed: f32,
};

@group(2) @binding(0) var<uniform> material: PlanetMaterial;

const PLANET_GLOW_THRESHOLD: f32 = 0.6;
const PLANET_GLOW_MULTIPLIER: f32 = 4.0;
const VORONOISE_U: f32 = 0.0;
const VORONOISE_V: f32 = 0.7;
const ATMOSPHERE_PLANET_MIX: f32 = 0.2;
const ATMOSPHERE_NOISE_SCALE: f32 = 16.0;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let len = length(in.uv - vec2f(0.5));

    var atmo: vec4f;

    if material.atmosphere_density != 0.0 {
        atmo = atmosphere(in.uv);
    } else {
        atmo = vec4f(0.0);
    }

    if len > material.planet_radius_normalized / 2.0 {
        return atmo;
    } else {
        return planet(in.uv) + atmo * ATMOSPHERE_PLANET_MIX;
    }
}

// Planet

fn planet(
    uv: vec2<f32>,
) -> vec4<f32> {

    var noise = 0.0;
    var max_i: u32;

    for (var i: u32 = 0; i < NB_COLORS; i++) {
        let seed = material.seed + f32(i);
        let t = random(seed) * 10.0;
        let n = max(0.1, nestedNoise(uv * t, seed));
        if n > noise {
            noise = n;
            max_i = i;
        }
    }

    var color = material.colors[max_i].xyz;

    for (var j: u32 = 0; j < NB_COLORS; j++) {
        if max_i != j {
            color += material.colors[j].xyz / f32(NB_COLORS);
        }
    }

    color *= noise;

    let glowFactor = max(0.0, (noise - PLANET_GLOW_THRESHOLD) / (1.0 - PLANET_GLOW_THRESHOLD));
    let glowColor = color * random(material.seed) * PLANET_GLOW_MULTIPLIER;
    color += glowColor * glowFactor;

    return vec4f(color, 1.0);
}

// Atmosphere

fn atmosphere(uv: vec2f) -> vec4f {
    let n: f32 = nestedMovingNoise(uv * ATMOSPHERE_NOISE_SCALE, material.atmosphere_speed, material.seed);

    let d = length(uv - vec2f(0.5)) * 2.0;

    return vec4<f32>(material.atmosphere_color.xyz * n, material.atmosphere_density * (1.0 - d));
}