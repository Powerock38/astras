use bevy::{
    ecs::spawn::SpawnWith, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef,
    sprite_render::Material2d,
};
use rand::prelude::*;

use crate::{
    MaterialLoader, MeshType,
    items::{ElementOnAstre, ElementState, Inventory},
    universe::{Astre, build_asteroid_belt, build_planet_group},
};

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect, Default)]
pub struct StarMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub seed: f32,
    #[uniform(0)]
    pub rotation: Vec2,
}

impl Material2d for StarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/star.wgsl".into()
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(Astre, MaterialLoader<StarMaterial>)]
pub struct Star;

pub fn build_star(rng: &mut StdRng, position: Vec2) -> impl Bundle + use<> {
    let radius = rng.random_range((10_000.)..30_000.);

    let nb_planets = rng.random_range(4..=15);

    let close_orbit = radius * 0.5;

    let orbit_distance = radius * 3.;

    let number_of_elements = rng.random_range(1..=3);

    let max_quantity = rng.random_range(100_000..=100_000_000);

    let composition = ElementOnAstre::random_elements(
        rng,
        number_of_elements,
        max_quantity,
        &[ElementState::Gas, ElementState::Plasma],
    );

    let color = ElementOnAstre::get_color(&composition);

    let rotation_direction =
        Vec2::new(rng.random_range(-1.0..=1.0), rng.random_range(-1.0..=1.0)).normalize();

    let rotation_speed = rng.random_range(0.001..=0.01);

    let material = StarMaterial {
        color,
        seed: rng.random::<f32>() * 1000.,
        rotation: rotation_direction * rotation_speed,
    };

    let mut rng = rng.clone();

    (
        Name::new("Star"),
        Star,
        Astre::new(radius, 0., close_orbit),
        Inventory::from(composition),
        MaterialLoader {
            material,
            mesh_type: MeshType::Circle(radius),
        },
        Transform::from_translation(position.extend(0.)),
        Children::spawn((SpawnWith(move |c: &mut ChildSpawner| {
            build_asteroid_belt(c, &mut rng);
            build_planet_group(c, &mut rng, radius / 2., orbit_distance, nb_planets, 0);
        }),)),
    )
}
