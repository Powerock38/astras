use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderRef;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use rand::Rng;

const BACKGROUND_Z: f32 = -999.;

#[derive(Component)]
pub struct Background;

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
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
            mesh: meshes.add(Mesh::from(Rectangle::default())).into(),
            material: materials.add(BackgroundMaterial {
                seed: rand::thread_rng().gen::<f32>() * 1000.,
            }),
            transform: Transform::from_translation(Vec3::new(0., 0., BACKGROUND_Z)),
            ..default()
        },
    ));
}
