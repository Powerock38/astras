use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d},
};
use rand::prelude::*;

use crate::{
    MaterialLoader, MeshType,
    data::ELEMENTS,
    items::{ElementOnAstre, ElementState, Inventory},
    universe::{Astre, Orbit},
};

pub const NB_COLORS: usize = 3;

pub type PlanetColors = [LinearRgba; NB_COLORS];

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(Astre, Orbit, MaterialLoader<PlanetMaterial>)]
pub struct Planet;

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect, Default)]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub seed: f32,
    #[uniform(0)]
    pub surface_colors: PlanetColors,
    #[uniform(0)]
    pub noise_scale: f32,
    #[uniform(0)]
    pub surface_ratio: f32,
    #[uniform(0)]
    pub shadow_angle: f32,
    #[uniform(0)]
    pub atmosphere_density: f32,
    #[uniform(0)]
    pub atmosphere_colors: PlanetColors,
    #[uniform(0)]
    pub atmosphere_speed: f32,
    #[uniform(0)]
    atmosphere_holes_threshold: f32,
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn build_planet_group(
    c: &mut ChildSpawner,
    rng: &mut StdRng,
    radius: f32,
    orbit_distance: f32,
    nb_children: u32,
    z_value: u32,
) {
    let mut orbit_distance = orbit_distance;

    for i in 0..nb_children {
        let c_nb_children = rng.random_range(0..=(0.1 * nb_children as f32) as u32);

        let c_angle = (i as f32 / nb_children as f32) * 2. * PI;

        let position = Vec2::new(
            orbit_distance * c_angle.cos(),
            orbit_distance * c_angle.sin(),
        );

        let r = build_planet(c, rng, radius, position, c_nb_children, z_value + i + 1);

        orbit_distance += r * 2.0;
    }
}

fn build_planet(
    c: &mut ChildSpawner,
    rng: &mut StdRng,
    parent_radius: f32,
    position: Vec2,
    nb_children: u32,
    z_value: u32,
) -> f32 {
    enum PlanetType {
        Classic,
        Airless,
        Gas,
    }

    let planet_type = [PlanetType::Classic, PlanetType::Airless, PlanetType::Gas]
        .choose(rng)
        .unwrap();

    let surface = match planet_type {
        PlanetType::Gas => 0.0,
        _ => {
            let s = rng.random_range((parent_radius * 0.1)..(parent_radius * 0.7));
            if s < 1000. {
                return 0.0;
            }
            s
        }
    };

    let atmosphere = match planet_type {
        PlanetType::Airless => 0.0,
        PlanetType::Gas => {
            let s = rng.random_range((parent_radius * 0.3)..(parent_radius * 0.9));
            if s < 1000. {
                return 0.0;
            }
            s
        }
        PlanetType::Classic => rng.random_range((surface * 0.3)..surface),
    };

    let planet_radius = surface + atmosphere;

    let close_orbit = rng.random_range((planet_radius * 0.5)..=(planet_radius * 0.8));

    let planet_total_radius = planet_radius + close_orbit;

    let surface_composition = match planet_type {
        PlanetType::Gas => vec![],
        _ => {
            let nb_surface_elements = rng.random_range(1..=ELEMENTS.len()) as u32;
            let max_quantity_surface_elements = rng.random_range(1_000..=1_000_000);

            ElementOnAstre::random_elements(
                rng,
                nb_surface_elements,
                max_quantity_surface_elements,
                &[ElementState::Solid, ElementState::Liquid],
            )
        }
    };

    let atmoshpere_composition = match planet_type {
        PlanetType::Airless => vec![],
        _ => {
            let nb_atmosphere_elements = rng.random_range(1..=5);
            let max_quantity_atmosphere_elements = rng.random_range(1_000..=100_000);

            ElementOnAstre::random_elements(
                rng,
                nb_atmosphere_elements,
                max_quantity_atmosphere_elements,
                &[ElementState::Gas],
            )
        }
    };

    let atmosphere_density = match planet_type {
        PlanetType::Airless => 0.0,
        PlanetType::Gas => rng.random_range(0.8..1.0),
        PlanetType::Classic => rng.random_range(0.01..0.3),
    };

    let atmosphere_speed = rng.random_range(0.01..0.2);

    let atmosphere_holes_threshold = rng.random_range(0..5) as f32 * 0.1;

    let surface_colors = ElementOnAstre::get_colors(&surface_composition);

    let atmosphere_colors = ElementOnAstre::get_colors(&atmoshpere_composition);

    let material = PlanetMaterial {
        seed: rng.random::<f32>() * 1000.,
        surface_colors,
        noise_scale: rng.random_range(1.0..10.0),
        surface_ratio: surface / planet_radius,
        shadow_angle: 0.0,
        atmosphere_density,
        atmosphere_colors,
        atmosphere_speed,
        atmosphere_holes_threshold,
    };

    c.spawn((
        Name::new("Planet"),
        Planet,
        Orbit::new(rng),
        Astre::new(surface, atmosphere, close_orbit),
        Inventory::from({
            let mut composition = surface_composition;
            composition.extend(atmoshpere_composition);
            composition
        }),
        MaterialLoader {
            material,
            mesh_type: MeshType::Circle(planet_radius),
        },
        Transform::from_translation(position.extend(z_value as f32)),
    ))
    .with_children(|c| {
        build_planet_group(
            c,
            rng,
            planet_radius,
            planet_total_radius * 3.0,
            nb_children,
            z_value,
        );
    });

    planet_total_radius
}

pub fn update_planet_shadows(
    mut materials: ResMut<Assets<PlanetMaterial>>,
    q_planets: Query<(&MeshMaterial2d<PlanetMaterial>, &GlobalTransform)>,
) {
    for (planet_material_handle, global_transform) in &q_planets {
        // Update shadow angle
        let material = materials.get_mut(planet_material_handle).unwrap();

        let star_position = Vec2::ZERO; // works well for now
        let planet_position = global_transform.translation().truncate();
        let delta = star_position - planet_position;

        material.shadow_angle = -delta.y.atan2(delta.x) + PI;
    }
}
