use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use rand::Rng;

use crate::{
    items::{ElementOnAstre, ElementState, Inventory},
    universe::{spawn_planet_c, Astre},
    HandleLoaderBundle, MaterialLoader,
};

#[derive(Bundle)]
pub struct StarBundle {
    pub star: Star,
    pub astre: Astre,
    pub inventory: Inventory,
    pub loader: HandleLoaderBundle<MaterialLoader<StarMaterial>>,
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

pub fn spawn_star(c: &mut ChildBuilder, radius: f32, position: Vec2, nb_children: u32) {
    let mut rng = rand::thread_rng();

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

    let rotation_direction =
        Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)).normalize();

    let rotation_speed = rng.gen_range(0.001..=0.01);

    let material = StarMaterial {
        color,
        seed: rng.gen::<f32>() * 1000.,
        rotation: rotation_direction * rotation_speed,
    };

    c.spawn(StarBundle {
        star: Star,
        astre: Astre::new(temperature, radius, 0.),
        inventory: Inventory::from(composition),
        loader: HandleLoaderBundle {
            loader: MaterialLoader { material, radius },
            transform,
            ..default()
        },
    })
    .with_children(|c| {
        spawn_planet_c(c, radius, orbit_distance, nb_children, 0);
    });
}
