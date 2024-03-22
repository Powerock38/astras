use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::Rng;

use crate::{
    astre::Astre,
    items::{ElementOnAstre, ElementState, Inventory},
    spawn_planet_c,
    utils::circle_mesh,
    PlanetMaterial,
};

#[derive(Bundle)]
pub struct StarBundle {
    pub star: Star,
    pub astre: Astre,
    pub inventory: Inventory,
    pub mesh: MaterialMesh2dBundle<StarMaterial>,
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct StarMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub seed: f32,
}

impl Material2d for StarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/star.wgsl".into()
    }
}

#[derive(Component)]
pub struct Star;

pub fn spawn_star(
    c: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StarMaterial>>,
    planet_materials: &mut ResMut<Assets<PlanetMaterial>>,
    radius: f32,
    position: Vec2,
    nb_children: u32,
) {
    let mut rng = rand::thread_rng();

    let mesh = circle_mesh(radius);

    let transform = Transform::from_translation(position.extend(0.));

    let orbit_distance = radius * 2.;

    let temperature = rng.gen_range(1_000..=1_000_000_000);

    let number_of_elements = rng.gen_range(1..=3);

    let composition = ElementOnAstre::random_elements(
        number_of_elements,
        rng.gen_range(100_000..=100_000_000),
        &[ElementState::Gas, ElementState::Plasma],
    );

    let color = ElementOnAstre::get_color(&composition);

    let material = StarMaterial {
        color,
        seed: rng.gen::<f32>() * 1000.,
    };

    c.spawn(StarBundle {
        star: Star,
        astre: Astre {
            temperature,
            surface_radius: radius,
            atmosphere_radius: 0.,
        },
        inventory: Inventory::from(composition),
        mesh: MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(material),
            transform,
            ..default()
        },
    })
    .with_children(|c| {
        spawn_planet_c(
            c,
            meshes,
            planet_materials,
            radius,
            orbit_distance,
            nb_children,
            0,
        );
    });
}
