use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use rand::prelude::*;

use crate::{
    items::{ElementOnAstre, ElementState},
    universe::{build_asteroid_belt, build_planet_children, AstreBundle},
    HandleLoaderBundle, MaterialLoader, MeshType,
};

#[derive(Bundle)]
pub struct StarBundle {
    name: Name,
    star: Star,
    astre_bundle: AstreBundle,
    loader: HandleLoaderBundle<MaterialLoader<StarMaterial>>,
}

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect, Default)]
pub struct StarMaterial {
    #[uniform(0)]
    pub color: Color,
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
#[reflect(Component)]
pub struct Star;

pub fn build_star(c: &mut ChildBuilder, rng: &mut StdRng, position: Vec2) {
    let transform = Transform::from_translation(position.extend(0.));

    let radius = rng.gen_range((5000.)..10000.);

    let nb_planets = rng.gen_range(5..=15);

    let orbit_distance = radius * 2.;

    let number_of_elements = rng.gen_range(1..=3);

    let max_quantity = rng.gen_range(100_000..=100_000_000);

    let composition = ElementOnAstre::random_elements(
        rng,
        number_of_elements,
        max_quantity,
        &[ElementState::Gas, ElementState::Plasma],
    );

    let color = ElementOnAstre::get_color(&composition);

    let rotation_direction =
        Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)).normalize();

    let rotation_speed = rng.gen_range(0.001..=0.01);

    let material = StarMaterial {
        color,
        seed: rng.gen::<f32>() * 1000.,
        rotation: rotation_direction * rotation_speed,
    };

    c.spawn(StarBundle {
        name: Name::new("Star"),
        star: Star,
        astre_bundle: AstreBundle::new(radius, 0., composition),
        loader: HandleLoaderBundle {
            loader: MaterialLoader {
                material,
                mesh_type: MeshType::Circle(radius),
            },
            transform,
            ..default()
        },
    })
    .with_children(|c| {
        build_planet_children(c, rng, radius, orbit_distance, nb_planets, 0);

        build_asteroid_belt(c, rng);
    });
}
