#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import "shaders/noise.wgsl"::{random, nestedNoise, nestedMovingNoise}

const NB_COLORS = 3u;

struct PlanetMaterial {
    seed: f32,
    colors: array<vec4<f32>, NB_COLORS>,
    noise_scale: f32,
    planet_radius_normalized: f32,
    shadow_angle: f32,
    atmosphere_density: f32,
    atmosphere_color: vec4<f32>,
    atmosphere_speed: f32,
    atmosphere_holes_threshold: f32,
};

@group(2) @binding(0) var<uniform> material: PlanetMaterial;

const PLANET_GLOW_THRESHOLD: f32 = 0.6;
const PLANET_GLOW_MULTIPLIER: f32 = 4.0;
const ATMOSPHERE_PLANET_MIX: f32 = 0.2;
const ATMOSPHERE_NOISE_SCALE: f32 = 16.0;
const SHADOW_RADIUS: f32 = 0.6;
const SHADOW_CENTER_OFFSET: f32 = 0.6;
const SHADOW_MIN_COLOR: f32 = 0.04;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let len = length(in.uv - vec2f(0.5));

    var color: vec3f;
    var alpha = 1.0;

    var atmo = vec3f(0.0);
    if material.atmosphere_density != 0.0 {
        atmo = atmosphere(in.uv);
    }

    if len > material.planet_radius_normalized / 2.0 {
        let d = length(in.uv - vec2f(0.5)) * 2.0;
        let atmo_alpha = material.atmosphere_density * (1.0 - d);
        color = atmo;
        alpha = atmo_alpha;
    } else {
        color = planet(in.uv) * (1.0 - ATMOSPHERE_PLANET_MIX) + atmo * ATMOSPHERE_PLANET_MIX;
    }

    // Shadow is a circle SDF
    let shadow = vec2f(cos(material.shadow_angle), sin(material.shadow_angle)) * SHADOW_CENTER_OFFSET + 0.5;
    color *= max(length(in.uv - shadow) - SHADOW_RADIUS, SHADOW_MIN_COLOR);

    return vec4<f32>(color, alpha);
}

// Planet

fn planet(
    uv: vec2<f32>,
) -> vec3<f32> {

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

    return color;
}

// Atmosphere

fn atmosphere(uv: vec2f) -> vec3f {
    var noise = nestedMovingNoise(uv * ATMOSPHERE_NOISE_SCALE, material.atmosphere_speed, material.seed);
    if noise < material.atmosphere_holes_threshold {
        noise = 0.0;
    }
    return material.atmosphere_color.xyz * noise;
}