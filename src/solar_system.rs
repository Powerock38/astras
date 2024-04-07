use bevy::prelude::*;
use rand::Rng;

use crate::{
    astres::{spawn_star, PlanetMaterial, StarMaterial},
    background::BackgroundMaterial,
    ship::setup_ship,
    worm::spawn_worm,
};

#[derive(Component)]
pub struct SolarSystem;

pub fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
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
            let nb_worms = 20;

            for _ in 0..nb_worms {
                let position = Vec2::new(
                    rand::thread_rng().gen_range(-radius..radius),
                    rand::thread_rng().gen_range(-radius..radius),
                );

                spawn_worm(c, &asset_server, position);
            }
        })
        .with_children(|c| {
            setup_ship(c, meshes, asset_server, background_materials);
        });
}
