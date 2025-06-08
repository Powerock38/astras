use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};
use rand::Rng;

use crate::MaterialLoader;

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(MaterialLoader<LaserMaterial>)]
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
    pub color: LinearRgba,
    #[uniform(0)]
    pub seed: f32,
}

impl Material2d for LaserMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/laser.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

impl LaserMaterial {
    pub fn new(color: LinearRgba) -> Self {
        LaserMaterial {
            color,
            seed: rand::rng().random(),
        }
    }
}

pub fn update_lasers(
    mut commands: Commands,
    time: Res<Time>,
    mut q_lasers: Query<(Entity, &mut Laser)>,
) {
    for (entity, mut laser) in &mut q_lasers {
        if laser.ttl.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}
