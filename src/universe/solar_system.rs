use bevy::prelude::*;
use rand::prelude::*;

use crate::universe::{build_ship, build_star, build_worm};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SolarSystem {
    position: [i32; 2],
}

impl SolarSystem {
    pub fn x(&self) -> i32 {
        self.position[0]
    }

    pub fn y(&self) -> i32 {
        self.position[1]
    }
}

pub fn solar_system_position_to_seed(position: [i32; 2]) -> u64 {
    let (x, y) = (position[0] as u64, position[1] as u64);
    let prime: u64 = 2_976_221_071;
    ((x.wrapping_mul(prime)).wrapping_add(y)) ^ 0x0005_DEEC_E66D
}

pub fn spawn_solar_system(commands: &mut Commands, position: [i32; 2]) {
    let seed = solar_system_position_to_seed(position);
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    commands
        .spawn((
            Name::new("SolarSytem"),
            SolarSystem { position },
            SpatialBundle::default(),
        ))
        .with_children(|c| {
            build_star(c, &mut rng, Vec2::ZERO);

            // Worms
            let radius = 50000.;
            let nb_worms = 3;

            for _ in 0..nb_worms {
                let worm_position = Vec2::new(
                    rng.gen_range(-radius..radius),
                    rng.gen_range(-radius..radius),
                );

                build_worm(c, &mut rng, worm_position);
            }
        })
        .with_children(|c| {
            build_ship(c);
        });
}
