use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::{
    universe::{build_background, Background, BackgroundMaterial, Ship},
    GameState,
};

const CAMERA_DOLLY_MAX_LENGTH: f32 = 0.05;
const CAMEAR_DOLLY_SPEED: f32 = 0.00001;
const CAMERA_ZOOM_SPEED: f32 = 2.0;
const CAMERA_CHANGE_LERP: f32 = 0.1;
const CAMERA_PAN_SPEED: f32 = 0.07;

const BASE_SCALE: f32 = 100.0;
const SWITCH_TO_PAN_MODE: f32 = 30.0;
const SWITCH_TO_UNIVERSE_MAP: f32 = 500.0;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(
    mut commands: Commands,
    ship: Single<Entity, Added<Ship>>,
    meshes: ResMut<Assets<Mesh>>,
    background_materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands.entity(*ship).with_children(|c| {
        c.spawn((
            Name::new("MainCamera"),
            MainCamera,
            Camera2d,
            Camera {
                hdr: true,
                ..default()
            },
            Projection::Orthographic(OrthographicProjection {
                scale: BASE_SCALE,
                near: -1000.0,
                far: 1000.0,
                ..OrthographicProjection::default_2d()
            }),
            Tonemapping::BlenderFilmic,
            Bloom::default(),
        ));

        c.spawn(build_background(meshes, background_materials));
    });
}

pub fn update_camera(
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut ev_motion: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    q_camera: Single<(&Camera, &GlobalTransform, &mut Projection), With<MainCamera>>,
    ship: Single<&Ship>,
    mut bg_transform: Single<&mut Transform, With<Background>>,
) {
    let (camera, global_transform, mut projection) = q_camera.into_inner();

    let scroll = ev_scroll.read().map(|scroll| scroll.y).sum::<f32>();
    let Projection::Orthographic(projection) = projection.as_mut() else {
        return;
    };
    projection.scale *= 1. - CAMERA_ZOOM_SPEED * scroll * time.delta_secs();

    if projection.scale > SWITCH_TO_UNIVERSE_MAP {
        next_state.set(GameState::GameUniverseMap);
        projection.scale = SWITCH_TO_UNIVERSE_MAP;
        return;
    }

    if projection.scale > SWITCH_TO_PAN_MODE {
        for motion in ev_motion.read() {
            if mouse_button_input.pressed(MouseButton::Left) {
                let mut delta = motion.delta * time.delta_secs() * CAMERA_PAN_SPEED;
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

    let viewport_position = Vec2::new(0.5 * window.width(), 0.5 * window.height());

    let Ok(position) = camera.viewport_to_world_2d(global_transform, viewport_position) else {
        return;
    };

    bg_transform.translation.x = position.x - global_transform.translation().x;
    bg_transform.translation.y = position.y - global_transform.translation().y;
    bg_transform.scale = Vec3::new(window.width(), window.height(), 0.0)
        * projection.scale
        * (1. + 2. * CAMERA_DOLLY_MAX_LENGTH)
        * 1.5;
}

pub fn reset_camera_viewport(q_projection: Single<&mut Projection, With<MainCamera>>) {
    let mut projection = q_projection.into_inner();
    let Projection::Orthographic(projection) = projection.as_mut() else {
        return;
    };
    projection.viewport_origin = Vec2::new(0.5, 0.5);
}
