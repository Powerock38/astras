use bevy::{ecs::spawn::SpawnIter, prelude::*};
use rand::prelude::*;

use crate::universe::{build_star, build_worm};

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct SolarSystem {
    pub position: [i32; 2],
}

impl SolarSystem {
    pub fn x(&self) -> i32 {
        self.position[0]
    }

    pub fn y(&self) -> i32 {
        self.position[1]
    }

    pub fn seed(&self) -> u64 {
        let (x, y) = (self.position[0] as u64, self.position[1] as u64);
        let prime = 2_976_221_071;
        ((x.wrapping_mul(prime)).wrapping_add(y)) ^ 0x0005_DEEC_E66D
    }
}

pub fn build_solar_system(position: [i32; 2]) -> impl Bundle {
    let solar_system = SolarSystem { position };
    let mut rng: StdRng = SeedableRng::seed_from_u64(solar_system.seed());

    let nb_worms = 3;

    (
        Name::new("SolarSytem"),
        solar_system,
        Transform::default(),
        Visibility::Visible,
        Children::spawn((
            Spawn(build_star(&mut rng, Vec2::ZERO)),
            SpawnIter((0..nb_worms).map(move |_| {
                let worm_spawn_radius = 50000.;
                let worm_position = Vec2::new(
                    rng.random_range(-worm_spawn_radius..worm_spawn_radius),
                    rng.random_range(-worm_spawn_radius..worm_spawn_radius),
                );
                build_worm(&mut rng, worm_position)
            })),
        )),
    )
}

#[derive(Component)]
pub struct ActiveSolarSystem;

pub fn set_active_solar_system(
    mut commands: Commands,
    q_active_solar_system: Query<Entity, With<ActiveSolarSystem>>,
    query: Query<(Entity, &InheritedVisibility), With<SolarSystem>>,
) {
    for active_entity in &q_active_solar_system {
        commands.entity(active_entity).remove::<ActiveSolarSystem>();
    }

    let Some(entity) = query
        .iter()
        .find(|(_, v)| v.get())
        .map(|(entity, _)| entity)
    else {
        error!("No visible solar system found");
        return;
    };

    commands.entity(entity).insert(ActiveSolarSystem);
}
