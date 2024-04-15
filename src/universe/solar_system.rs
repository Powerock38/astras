use bevy::prelude::*;
use rand::Rng;

use crate::universe::{build_ship, build_star, build_worm};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SolarSystem;

pub fn spawn_solar_system(commands: &mut Commands) {
    let mut rng = rand::thread_rng();

    let star_radius = rng.gen_range((5000.)..10000.);
    let star_position = Vec2::new(0., 0.);
    let nb_planets = rng.gen_range(5..=15);

    commands
        .spawn(SpatialBundle::default())
        .insert(SolarSystem)
        .with_children(|mut c| {
            build_star(&mut c, star_radius, star_position, nb_planets);
        })
        .with_children(|c| {
            let radius = 50000.;
            let nb_worms = 3;

            for _ in 0..nb_worms {
                let position = Vec2::new(
                    rand::thread_rng().gen_range(-radius..radius),
                    rand::thread_rng().gen_range(-radius..radius),
                );

                build_worm(c, position);
            }
        })
        .with_children(|c| {
            build_ship(c);
        });
}
