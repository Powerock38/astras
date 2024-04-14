use bevy::prelude::*;
use rand::Rng;

use crate::{astres::spawn_star, ship::setup_ship, worm::spawn_worm};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SolarSystem;

// TODO: save and load on new game
pub fn spawn_solar_system(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let radius = rng.gen_range((5000.)..10000.);
    let nb_children = rng.gen_range(5..=15);

    let position = Vec2::new(0., 0.);

    commands
        .spawn(SpatialBundle::default())
        .insert(SolarSystem)
        .with_children(|mut c| {
            spawn_star(&mut c, radius, position, nb_children);
        })
        .with_children(|c| {
            let radius = 50000.;
            let nb_worms = 3;

            for _ in 0..nb_worms {
                let position = Vec2::new(
                    rand::thread_rng().gen_range(-radius..radius),
                    rand::thread_rng().gen_range(-radius..radius),
                );

                spawn_worm(c, position);
            }
        })
        .with_children(|c| {
            setup_ship(c);
        });
}
