use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
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

pub fn build_background(
    c: &mut ChildBuilder,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    c.spawn((
        Background,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(BackgroundMaterial {
            seed: rand::rng().random::<f32>() * 1000.,
        })),
        Transform::from_translation(Vec3::new(0., 0., BACKGROUND_Z)),
    ));
}
