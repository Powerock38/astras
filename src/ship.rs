use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::background::*;
use crate::dockable_on_astre::DockableOnAstre;

const CAMERA_DOLLY_MAX_LENGTH: f32 = 0.05;
pub const SHIP_Z: f32 = 100.;

#[derive(Component)]
pub struct Ship {
    speed: Vec2,
    max_speed: f32,
    thrust: f32,
}

#[derive(Component)]
pub struct ShipSprite;

pub fn setup_ship(
    c: &mut ChildBuilder,
    meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    background_materials: ResMut<Assets<BackgroundMaterial>>,
) {
    let position = Vec2::new(0., 0.);

    // Ship is just a SpatialBundle
    c.spawn((
        Ship {
            speed: Vec2::new(0., 0.),
            max_speed: 50.,
            thrust: 3000.,
        },
        DockableOnAstre::default(),
        SpatialBundle {
            transform: Transform::from_translation(position.extend(1.)),
            ..default()
        },
    ))
    .with_children(|c| {
        // Ship sprite as a child of ship, so we can rotate the sprite without rotating camera
        c.spawn((
            ShipSprite,
            SpriteBundle {
                texture: asset_server.load("sprites/ship-simple-downscale.png"),
                ..default()
            },
        ));

        // Camera as a child of ship, so it follows the ship
        c.spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., SHIP_Z)),
            projection: OrthographicProjection {
                scale: 5.,
                ..default()
            },
            ..default()
        });

        // same for background
        spawn_background(c, meshes, background_materials);
    });
}

pub fn update_ship(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<(&mut Ship, &mut Transform), Without<ShipSprite>>,
    mut q_ship_sprite: Query<&mut Transform, With<ShipSprite>>,
) {
    for (mut ship, mut transform) in q_ship.iter_mut() {
        let mut movement = Vec2::new(0., 0.);

        if keyboard_input.pressed(KeyCode::Space) {
            ship.speed *= 0.9;
        }

        if keyboard_input.any_pressed(vec![KeyCode::ArrowLeft, KeyCode::KeyA]) {
            movement.x -= 1.;
        }
        if keyboard_input.any_pressed(vec![KeyCode::ArrowRight, KeyCode::KeyD]) {
            movement.x += 1.;
        }
        if keyboard_input.any_pressed(vec![KeyCode::ArrowUp, KeyCode::KeyW]) {
            movement.y += 1.;
        }
        if keyboard_input.any_pressed(vec![KeyCode::ArrowDown, KeyCode::KeyS]) {
            movement.y -= 1.;
        }

        let acceleration = movement * ship.thrust;

        let max_speed = ship.max_speed;

        ship.speed += (acceleration * time.delta_seconds()).clamp_length_max(max_speed);

        transform.translation += (ship.speed * time.delta_seconds()).extend(0.);

        for mut sprite_transform in q_ship_sprite.iter_mut() {
            sprite_transform.rotation = Quat::from_rotation_z(-ship.speed.angle_between(Vec2::Y));
        }
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

    for scroll in scroll_evr.read() {
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
