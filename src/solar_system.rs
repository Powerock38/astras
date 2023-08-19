use bevy::prelude::*;
use rand::Rng;

use crate::{astre::spawn_astre, ship::setup_ship, worm::spawn_worm};

#[derive(Component)]
pub struct SolarSystem;

pub fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = rand::thread_rng().gen_range((300.)..1000.);
    let mass = rand::thread_rng().gen_range((100.)..3000.);
    let position = Vec2::new(0., 0.);
    let nb_children = rand::thread_rng().gen_range(3..=20);

    commands
        .spawn(SpatialBundle::default())
        .insert(SolarSystem)
        .with_children(|mut c| {
            spawn_astre(
                &mut c,
                &mut meshes,
                &mut materials,
                0.,
                radius,
                mass,
                position,
                0.,
                true,
                nb_children,
                0,
            );
        })
        .with_children(|c| {
            let radius = 10000.;

            for _ in 0..10 {
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
                    &mut materials,
                    position,
                    size,
                    speed,
                    length,
                    change_direction_every,
                );
            }
        })
        .with_children(|c| {
            setup_ship(c, meshes, materials);
        });
}