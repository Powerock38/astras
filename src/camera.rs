use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::universe::{build_background, Background, BackgroundMaterial, Ship};

const CAMERA_DOLLY_MAX_LENGTH: f32 = 0.05;
const CAMEAR_DOLLY_SPEED: f32 = 0.00001;
const CAMERA_ZOOM_SPEED: f32 = 2.0;
const CAMERA_CHANGE_LERP: f32 = 0.1;
const CAMERA_PAN_MODE_THRESHOLD: f32 = 30.0;
const CAMERA_PAN_SPEED: f32 = 0.1;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(
    mut commands: Commands,
    q_ship: Query<Entity, Added<Ship>>,
    meshes: ResMut<Assets<Mesh>>,
    background_materials: ResMut<Assets<BackgroundMaterial>>,
) {
    let Some(ship) = q_ship.iter().next() else {
        return;
    };

    commands.entity(ship).with_children(|c| {
        c.spawn((
            MainCamera,
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

        build_background(c, meshes, background_materials);
    });
}

pub fn update_camera(
    time: Res<Time>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut ev_motion: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut q_projection: Query<
        (&Camera, &GlobalTransform, &mut OrthographicProjection),
        With<MainCamera>,
    >,
    mut q_background: Query<&mut Transform, With<Background>>,
    q_ship: Query<&Ship>,
    window: Query<&Window>,
) {
    let window = window.single();
    let Some((camera, global_transform, mut projection)) = q_projection.iter_mut().next() else {
        return;
    };

    let Some(ship) = q_ship.iter().next() else {
        return;
    };

    for scroll in ev_scroll.read() {
        projection.scale *= 1. - CAMERA_ZOOM_SPEED * scroll.y * time.delta_seconds();
    }

    if projection.scale > CAMERA_PAN_MODE_THRESHOLD {
        for motion in ev_motion.read() {
            if mouse_button_input.pressed(MouseButton::Left) {
                let mut delta = motion.delta.normalize() * time.delta_seconds() * CAMERA_PAN_SPEED;
                delta.y *= -1.;
                projection.viewport_origin += delta;
            }
        }
    } else {
        let dolly_offset =
            (ship.speed() * CAMEAR_DOLLY_SPEED).clamp_length_max(CAMERA_DOLLY_MAX_LENGTH);
        projection.viewport_origin = projection
            .viewport_origin
            .lerp(dolly_offset + Vec2::new(0.5, 0.5), CAMERA_CHANGE_LERP);
    }

    let Some(mut bg_transform) = q_background.iter_mut().next() else {
        return;
    };

    let viewport_position = Vec2::new(0.5 * window.width(), 0.5 * window.height());

    let Some(position) = camera.viewport_to_world_2d(global_transform, viewport_position) else {
        return;
    };

    bg_transform.translation.x = position.x - global_transform.translation().x;
    bg_transform.translation.y = position.y - global_transform.translation().y;
    bg_transform.scale = Vec3::new(window.width(), window.height(), 0.0)
        * projection.scale
        * (1. + 2. * CAMERA_DOLLY_MAX_LENGTH);
}
