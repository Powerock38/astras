use bevy::{
    core_pipeline::clear_color::ClearColorConfig, input::mouse::MouseWheel, prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::background::*;
use crate::dockable_on_astre::DockableOnAstre;

const CAMERA_DOLLY_MAX_LENGTH: f32 = 0.05;

#[derive(Component)]
pub struct Ship {
    speed: Vec2,
    thrust: f32,
}

pub fn setup_ship(
    c: &mut ChildBuilder,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    background_materials: ResMut<Assets<BackgroundMaterial>>,
) {
    let position = Vec2::new(0., 0.);

    c.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(), // Triangle
        material: materials.add(ColorMaterial::from(Color::GOLD)),
        transform: Transform::from_translation(position.extend(1.)),
        ..default()
    })
    .insert((
        Ship {
            speed: Vec2::new(0., 0.),
            thrust: 3000.,
        },
        DockableOnAstre::default(),
    ))
    .with_children(|c| {
        c.spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
            projection: OrthographicProjection {
                scale: 5.,
                ..default()
            },
            ..default()
        });

        spawn_background(c, meshes, background_materials);
    });
}

pub fn update_ship(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Ship, &mut Transform)>,
) {
    for (mut ship, mut transform) in query.iter_mut() {
        let mut movement = Vec2::new(0., 0.);

        if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::Q]) {
            movement.x -= 1.;
        }
        if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D]) {
            movement.x += 1.;
        }
        if keyboard_input.any_pressed(vec![KeyCode::Up, KeyCode::Z]) {
            movement.y += 1.;
        }
        if keyboard_input.any_pressed(vec![KeyCode::Down, KeyCode::S]) {
            movement.y -= 1.;
        }

        let acceleration = movement * ship.thrust;

        ship.speed += acceleration * time.delta_seconds();

        transform.translation += (ship.speed * time.delta_seconds()).extend(0.);
    }
}

pub fn update_camera(
    time: Res<Time>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    mut background_query: Query<&mut Transform, With<Background>>,
    ship_query: Query<&Ship>,
    window: Query<&Window>,
) {
    let window = window.single();

    let mut projection = query.single_mut();

    for scroll in scroll_evr.iter() {
        projection.scale *= 1. + 2. * scroll.y * time.delta_seconds();
    }

    for mut transform in background_query.iter_mut() {
        transform.scale = Vec3::new(window.width(), window.height(), 0.0)
            * projection.scale
            * (1. + 2. * CAMERA_DOLLY_MAX_LENGTH);
    }

    for ship in ship_query.iter() {
        let dolly_offset = (ship.speed * 0.00001).clamp_length_max(CAMERA_DOLLY_MAX_LENGTH);
        projection.viewport_origin = dolly_offset + Vec2::new(0.5, 0.5);
    }
}
