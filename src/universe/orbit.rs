use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Orbit {
    speed: f32,
}

impl Orbit {
    pub fn new(rng: &mut StdRng) -> Self {
        let speed =
            rng.gen_range((PI / 1000.)..=PI / 100.) * if rng.gen_bool(0.5) { 1. } else { -1. }; // * random direction
        Self { speed }
    }
}

pub fn update_orbits(time: Res<Time>, mut query: Query<(&Orbit, &mut Transform)>) {
    for (orbit, mut transform) in &mut query {
        let angle = transform.translation.y.atan2(transform.translation.x);
        let distance = transform.translation.distance(Vec3::ZERO);

        let orbit_angle = angle + orbit.speed * time.delta_seconds();

        transform.translation.x = distance * orbit_angle.cos();
        transform.translation.y = distance * orbit_angle.sin();
    }
}
