use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::{
    reflect::TypeUuid,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::Rng;

use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
pub struct BackgroundMaterial {
    #[uniform(0)]
    pub seed: f32,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}

pub fn spawn_background(
    c: &mut ChildBuilder,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    c.spawn((
        Background,
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -100.0),
                ..default()
            },
            material: materials.add(BackgroundMaterial {
                seed: rand::thread_rng().gen::<f32>() * 1000.,
            }),
            ..default()
        },
    ));
}
