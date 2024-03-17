use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::prelude::SliceRandom;

use crate::{constants::COLORS, spawn_planet_c, PlanetMaterial};

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct StarMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material2d for StarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/star.wgsl".into()
    }
}

#[derive(Component)]
pub struct Star {
    pub radius: f32,
}

pub fn spawn_star(
    c: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StarMaterial>>,
    planet_materials: &mut ResMut<Assets<PlanetMaterial>>,
    radius: f32,
    position: Vec2,
    nb_children: u32,
) {
    let mut rng = rand::thread_rng();

    let color = COLORS.choose(&mut rng).unwrap();

    let material = StarMaterial { color: *color };

    let mesh = Circle::new(radius);

    let transform = Transform::from_translation(position.extend(0.));

    let star = Star { radius };

    let orbit_distance = radius * 2.;

    c.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: materials.add(material),
        transform,
        ..default()
    })
    .insert(star)
    .with_children(|c| {
        spawn_planet_c(
            c,
            meshes,
            planet_materials,
            radius,
            orbit_distance,
            nb_children,
            0,
        );
    });
}
