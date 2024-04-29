use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use rand::Rng;

use crate::{HandleLoaderBundle, MaterialLoader};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Laser {
    ttl: Timer,
}

impl Laser {
    pub fn new(seconds: f32) -> Self {
        Self {
            ttl: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
}

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect, Default)]
pub struct LaserMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub seed: f32,
}

impl Material2d for LaserMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/laser.wgsl".into()
    }
}

impl LaserMaterial {
    pub fn new(color: Color) -> Self {
        let mut rng = rand::thread_rng();

        LaserMaterial {
            color,
            seed: rng.gen(),
        }
    }
}

#[derive(Bundle)]
pub struct LaserBundle {
    pub laser: Laser,
    pub loader: HandleLoaderBundle<MaterialLoader<LaserMaterial>>,
}

pub fn update_lasers(
    mut commands: Commands,
    time: Res<Time>,
    mut q_lasers: Query<(Entity, &mut Laser)>,
) {
    for (entity, mut laser) in q_lasers.iter_mut() {
        if laser.ttl.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
