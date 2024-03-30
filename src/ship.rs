use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
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
            transform: Transform::from_translation(position.extend(SHIP_Z)),
            ..default()
        },
    ))
    .with_children(|c| {
        // Ship sprite as a child of ship, so we can rotate the sprite without rotating camera
        c.spawn((
            ShipSprite,
            SpriteBundle {
                texture: asset_server.load("sprites/ship.png"),
                ..default()
            },
        ));

        // Camera as a child of ship, so it follows the ship
        c.spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                projection: OrthographicProjection {
                    scale: 100.0,
                    ..default()
                },
                tonemapping: Tonemapping::BlenderFilmic,
                ..default()
            },
            BloomSettings::default(),
        ));

        // same for background
        spawn_background(c, meshes, background_materials);
    });
}

pub fn update_ship(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<(&mut Ship, &mut Transform, &DockableOnAstre), Without<ShipSprite>>,
    mut q_ship_sprite: Query<&mut Transform, With<ShipSprite>>,
) {
    for (mut ship, mut transform, dockable) in q_ship.iter_mut() {
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

        if dockable.is_docked() {
            ship.speed *= 0.99;
        }

        if ship.speed.length() < 0.001 {
            ship.speed = Vec2::ZERO;
        }

        transform.translation.x += ship.speed.x * time.delta_seconds();
        transform.translation.y += ship.speed.y * time.delta_seconds();

        // Sprite rotation
        let mut sprite_transform = q_ship_sprite.single_mut();
        if ship.speed == Vec2::ZERO {
            sprite_transform.rotation = Quat::from_rotation_z(0.);
        } else {
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
        projection.scale *= 1. - 2. * scroll.y * time.delta_seconds();
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
