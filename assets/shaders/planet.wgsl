#import bevy_pbr::forward_io::VertexOutput
#import "shaders/noise.wgsl"::{nestedNoise, voroNoise2}

struct PlanetMaterial {
    color: vec4<f32>,
    seed: f32,
    noise_scale: f32,
    planet_radius_normalized: f32,
    atmosphere_density: f32,
    atmosphere_color: vec4<f32>,
    atmosphere_speed: f32,
};

@group(2) @binding(0) var<uniform> material: PlanetMaterial;

const PLANET_GLOW_THRESHOLD: f32 = 0.7;
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

    let atmo = atmosphere(in.uv);

    if (len > material.planet_radius_normalized / 2.0) {
        return atmo;
    } else {
        return planet(in.uv) + atmo * ATMOSPHERE_PLANET_MIX;
    }
}

// Planet

fn planet(
  uv: vec2<f32>,
) -> vec4<f32> {
    let noise = voroNoise2((uv + material.seed) * material.noise_scale, VORONOISE_U, VORONOISE_V);
    var color = material.color.xyz * max(0.1, noise * 0.5);

    // Calculate the glow factor based on the brightness of the noise
    let glowFactor = max(0.0, (noise - PLANET_GLOW_THRESHOLD) / (1.0 - PLANET_GLOW_THRESHOLD));
    let glowColor = material.color.xyz * PLANET_GLOW_MULTIPLIER;
    color += glowColor * glowFactor;

    return vec4f(color, 1.0);
}

// Atmosphere

fn atmosphere(uv: vec2f) -> vec4f {
    let n: f32 = nestedNoise(uv * ATMOSPHERE_NOISE_SCALE, material.atmosphere_speed);

    let d = length(uv - vec2f(0.5)) / 0.5;

    return vec4<f32>(material.atmosphere_color.xyz * n, material.atmosphere_density * (1.0 - d));
}