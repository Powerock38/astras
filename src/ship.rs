use bevy::{
    core_pipeline::clear_color::ClearColorConfig, input::mouse::MouseWheel, prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::background::*;
use crate::dockable_on_astre::DockableOnAstre;

#[derive(Component)]
pub struct Ship {
    speed: f32,
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
            speed: 3000., // pixels per second
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
    mut query: Query<(&Ship, &mut Transform)>,
) {
    for (ship, mut transform) in query.iter_mut() {
        let mut movement = Vec3::new(0., 0., 0.);

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

        transform.translation += movement * ship.speed * time.delta_seconds();
    }
}

pub fn update_camera(
    time: Res<Time>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    mut background_query: Query<&mut Transform, With<Background>>,
    window: Query<&Window>,
) {
    let window = window.single();

    for scroll in scroll_evr.iter() {
        for mut projection in query.iter_mut() {
            projection.scale *= 1. + 2. * scroll.y * time.delta_seconds();

            for mut transform in background_query.iter_mut() {
                transform.scale =
                    Vec3::new(window.width(), window.height(), 0.0) * projection.scale;
            }
        }
    }
}
