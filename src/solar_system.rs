use bevy::prelude::*;
use rand::Rng;

use crate::{
    background::BackgroundMaterial, planet::PlanetMaterial, ship::setup_ship, spawn_star,
    worm::spawn_worm, StarMaterial,
};

#[derive(Component)]
pub struct SolarSystem;

pub fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut star_materials: ResMut<Assets<StarMaterial>>,
    mut planet_materials: ResMut<Assets<PlanetMaterial>>,
    background_materials: ResMut<Assets<BackgroundMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    let radius = rng.gen_range((5000.)..10000.);
    let nb_children = rng.gen_range(5..=15);

    let position = Vec2::new(0., 0.);

    commands
        .spawn(SpatialBundle::default())
        .insert(SolarSystem)
        .with_children(|mut c| {
            spawn_star(
                &mut c,
                &mut meshes,
                &mut star_materials,
                &mut planet_materials,
                radius,
                position,
                nb_children,
            );
        })
        .with_children(|c| {
            let radius = 10000.;
            let nb_worms = 0;

            for _ in 0..nb_worms {
                let position = Vec2::new(
                    rand::thread_rng().gen_range(-radius..radius),
                    rand::thread_rng().gen_range(-radius..radius),
                );

                let size = rand::thread_rng().gen_range(10. ..=100.);
                let speed = rand::thread_rng().gen_range(100. ..=1000.);
                let length = rand::thread_rng().gen_range(1..=10);

                let change_direction_every = rand::thread_rng().gen_range(0.1..=3.);

                spawn_worm(
                    c,
                    &mut meshes,
                    &mut color_materials,
                    position,
                    size,
                    speed,
                    length,
                    change_direction_every,
                );
            }
        })
        .with_children(|c| {
            setup_ship(c, meshes, asset_server, background_materials);
        });
}
