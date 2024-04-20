use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use rand::Rng;
use std::f32::consts::PI;

use crate::{
    items::{ElementOnAstre, ElementState, Inventory, ELEMENTS},
    universe::{Astre, Star},
    HandleLoaderBundle, MaterialLoader,
};

pub const NB_COLORS: usize = 3;

pub type PlanetColors = [Color; NB_COLORS];

#[derive(Bundle)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub astre: Astre,
    pub inventory: Inventory,
    pub loader: HandleLoaderBundle<MaterialLoader<PlanetMaterial>>,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Planet {
    pub orbit_speed: f32,
}

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect, Default)]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub seed: f32,
    #[uniform(0)]
    pub colors: PlanetColors,
    #[uniform(0)]
    pub noise_scale: f32,
    #[uniform(0)]
    pub planet_radius_normalized: f32,
    #[uniform(0)]
    pub shadow_angle: f32,
    #[uniform(0)]
    pub atmosphere_density: f32,
    #[uniform(0)]
    pub atmosphere_color: Color,
    #[uniform(0)]
    pub atmosphere_speed: f32,
    #[uniform(0)]
    atmosphere_holes_threshold: f32,
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
}

pub fn build_planet(
    c: &mut ChildBuilder,
    surface_radius: f32,
    atmosphere_radius: f32,
    position: Vec2,
    orbit_speed: f32,
    nb_children: u32,
    z_value: u32,
) {
    let total_radius = atmosphere_radius + surface_radius;

    let mut rng = rand::thread_rng();

    let transform = Transform::from_translation(position.extend(z_value as f32));

    let orbit_distance = total_radius * 2.0;

    let temperature = rng.gen_range(20..=1000);

    let mut composition = ElementOnAstre::random_elements(
        rng.gen_range(1..=ELEMENTS.len()) as u32,
        rng.gen_range(1_000..=1_000_000),
        &[ElementState::Solid, ElementState::Liquid],
    );

    let no_atmosphere = atmosphere_radius == 0.0;

    let atmoshpere_composition = if no_atmosphere {
        vec![]
    } else {
        ElementOnAstre::random_elements(
            rng.gen_range(1..=5),
            rng.gen_range(1_000..=100_000),
            &[ElementState::Gas],
        )
    };

    let atmosphere_density = if no_atmosphere {
        0.0
    } else {
        rng.gen_range(0.01..0.5)
    };

    let atmosphere_speed = rng.gen_range(0.01..1.0);

    let atmosphere_holes_threshold = rng.gen_range(0..5) as f32 * 0.1;

    let colors = ElementOnAstre::get_colors(&composition);

    let atmosphere_color = ElementOnAstre::get_color(&atmoshpere_composition);

    composition.extend(atmoshpere_composition);

    let material = PlanetMaterial {
        seed: rng.gen::<f32>() * 1000.,
        colors,
        noise_scale: rng.gen_range(1.0..10.0),
        planet_radius_normalized: surface_radius / total_radius,
        shadow_angle: 0.0,
        atmosphere_density,
        atmosphere_color,
        atmosphere_speed,
        atmosphere_holes_threshold,
    };

    c.spawn(PlanetBundle {
        planet: Planet { orbit_speed },
        astre: Astre::new(temperature, surface_radius, atmosphere_radius),
        inventory: Inventory::from(composition),
        loader: HandleLoaderBundle {
            loader: MaterialLoader {
                material,
                radius: total_radius,
            },
            transform,
            ..default()
        },
    })
    .with_children(|c| {
        build_planet_children(c, surface_radius, orbit_distance, nb_children, z_value);
    });
}

pub fn build_planet_children(
    c: &mut ChildBuilder,
    surface_radius: f32,
    mut orbit_distance: f32,
    nb_children: u32,
    z_value: u32,
) {
    let mut rng = rand::thread_rng();

    for i in 0..nb_children {
        let c_surface_radius = rng.gen_range((surface_radius * 0.1)..(surface_radius * 0.9));

        if c_surface_radius < 1000. {
            continue;
        }

        let c_no_atmosphere = rng.gen_bool(0.3);

        let c_atmosphere_radius = if c_no_atmosphere {
            0.0
        } else {
            rng.gen_range(0.0..(c_surface_radius * 0.5))
        };

        let c_nb_children = rng.gen_range(0..=(0.1 * nb_children as f32) as u32);

        let c_angle = (i as f32 / nb_children as f32) * 2. * PI;

        let c_orbit_speed = rng.gen_range((PI / 100.)..=PI / 50.)
                // * random direction
                * if rng.gen_bool(0.5) { 1. } else { -1. };

        orbit_distance += c_surface_radius + c_atmosphere_radius;

        let position = Vec2::new(
            orbit_distance * c_angle.cos(),
            orbit_distance * c_angle.sin(),
        );

        build_planet(
            c,
            c_surface_radius,
            c_atmosphere_radius,
            position,
            c_orbit_speed,
            c_nb_children,
            z_value + i + 1,
        );

        orbit_distance += rng.gen_range((c_atmosphere_radius * 0.2)..=(c_atmosphere_radius * 1.5));
    }
}

pub fn update_planets(
    time: Res<Time>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
    mut q_planets: Query<(
        &Planet,
        &Handle<PlanetMaterial>,
        &mut Transform,
        &GlobalTransform,
    )>,
    q_stars: Query<&GlobalTransform, With<Star>>,
) {
    for (planet, planet_material_handle, mut transform, global_transform) in q_planets.iter_mut() {
        let angle = transform.translation.y.atan2(transform.translation.x);
        let orbit = (transform.translation.x.powf(2.0) + transform.translation.y.powf(2.0)).sqrt();

        let orbit_angle = angle + planet.orbit_speed * time.delta_seconds();

        transform.translation.x = orbit * orbit_angle.cos();
        transform.translation.y = orbit * orbit_angle.sin();

        // Update shadow angle
        let material = materials.get_mut(planet_material_handle).unwrap();

        //TODO maybe support multiple stars
        let Some(star_global_transform) = q_stars.iter().next() else {
            return;
        };

        let star_position = star_global_transform.translation().truncate();
        let planet_position = global_transform.translation().truncate();
        let delta = star_position - planet_position;

        material.shadow_angle = -delta.y.atan2(delta.x) + PI;
    }
}
