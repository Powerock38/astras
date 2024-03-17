use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::{seq::SliceRandom, Rng};
use std::f32::consts::PI;

use crate::constants::COLORS;

#[derive(Component, Debug)]
pub struct Planet {
    pub atmosphere_radius: f32,
    pub surface_radius: f32,
    pub orbit_speed: f32,
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub seed: f32,
    #[uniform(0)]
    pub noise_scale: f32,
    #[uniform(0)]
    pub planet_radius_normalized: f32,
    #[uniform(0)]
    pub atmosphere_density: f32,
    #[uniform(0)]
    pub atmosphere_color: Color,
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
}

pub fn spawn_planet(
    c: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<PlanetMaterial>>,
    surface_radius: f32,
    atmosphere_radius: f32,
    position: Vec2,
    orbit_speed: f32,
    nb_children: u32,
    z_value: u32,
) {
    let total_radius = atmosphere_radius + surface_radius;

    let mut rng = rand::thread_rng();

    let planet_color = COLORS.choose(&mut rng).unwrap();
    let atmosphere_color = COLORS.choose(&mut rng).unwrap();

    let material = PlanetMaterial {
        color: *planet_color,
        seed: rng.gen::<f32>() * 1000.,
        noise_scale: rng.gen_range(1.0..10.0),
        planet_radius_normalized: surface_radius / total_radius,
        atmosphere_density: rng.gen_range(0.01..0.5),
        atmosphere_color: *atmosphere_color,
    };

    let mesh = Circle::new(total_radius);

    let transform = Transform::from_translation(position.extend(z_value as f32));

    let planet = Planet {
        atmosphere_radius,
        surface_radius,
        orbit_speed,
    };

    let orbit_distance = total_radius * 2.0;

    c.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: materials.add(material),
        transform,
        ..default()
    })
    .insert(planet)
    .with_children(|c| {
        spawn_planet_c(
            c,
            meshes,
            materials,
            surface_radius,
            orbit_distance,
            nb_children,
            z_value,
        );
    });
}

pub fn spawn_planet_c(
    c: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<PlanetMaterial>>,
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

        let c_atmosphere_radius = rng.gen_range((c_surface_radius * 0.5)..(c_surface_radius * 0.9));

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

        spawn_planet(
            c,
            meshes,
            materials,
            c_surface_radius,
            c_atmosphere_radius,
            position,
            c_orbit_speed,
            c_nb_children,
            z_value + 1,
        );

        orbit_distance += rng.gen_range((c_atmosphere_radius * 0.2)..=(c_atmosphere_radius * 1.5));
    }
}

pub fn update_planets(time: Res<Time>, mut q_planets: Query<(&Planet, &mut Transform)>) {
    for (planet, mut transform) in q_planets.iter_mut() {
        let angle = transform.translation.y.atan2(transform.translation.x);
        let orbit = (transform.translation.x.powf(2.0) + transform.translation.y.powf(2.0)).sqrt();

        let orbit_angle = angle + planet.orbit_speed * time.delta_seconds();

        transform.translation =
            Vec2::new(orbit * orbit_angle.cos(), orbit * orbit_angle.sin()).extend(0.);
    }
}
