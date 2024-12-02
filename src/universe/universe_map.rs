use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use rand::prelude::*;

use crate::{
    universe::{build_star, solar_system_position_to_seed, MainCamera, SolarSystem},
    GameState,
};

const OBSERVABLE_UNIVERSE_RADIUS: i32 = 5;
const SOLAR_SYSTEMS_SPACING: f32 = 1_000_000.0;
const POSITION_MAX_OFFSET: f32 = SOLAR_SYSTEMS_SPACING * 0.5;
const SOLAR_SYSTEMS_SCALE: f32 = 0.1;
const BASE_SCALE: f32 = 600.0;
const PAN_SPEED: f32 = 0.1;
const PAN_KEYBOARD_SPEED: f32 = 1.0;
const ZOOM_SPEED: f32 = 2.0;
const SWITCH_TO_SOLAR_SYSTEM: f32 = 500.0;

#[derive(Component)]
pub struct UniverseMap;

#[derive(Component)]
pub struct UniverseMapCamera;

pub fn spawn_universe_map(
    mut commands: Commands,
    mut q_main_camera: Query<&mut Camera, With<MainCamera>>,
    mut q_solar_systems: Query<(&SolarSystem, &mut Visibility)>,
) {
    if let Ok(mut camera) = q_main_camera.get_single_mut() {
        camera.is_active = false;
    }

    let Ok((solar_system, mut solar_system_visibility)) = q_solar_systems.get_single_mut() else {
        return;
    };

    *solar_system_visibility = Visibility::Hidden;

    commands
        .spawn((
            Name::new("UniverseMap"),
            UniverseMap,
            Transform::from_scale(Vec3::splat(SOLAR_SYSTEMS_SCALE)),
            Visibility::default(),
        ))
        .with_children(|c| {
            let x_min = solar_system.x() - OBSERVABLE_UNIVERSE_RADIUS / 2;
            let x_max = solar_system.x() + OBSERVABLE_UNIVERSE_RADIUS / 2;
            let y_min = solar_system.y() - OBSERVABLE_UNIVERSE_RADIUS / 2;
            let y_max = solar_system.y() + OBSERVABLE_UNIVERSE_RADIUS / 2;

            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    let seed = solar_system_position_to_seed([x, y]);

                    let mut rng = StdRng::seed_from_u64(seed);

                    let x_offset = rng.gen_range(-POSITION_MAX_OFFSET..POSITION_MAX_OFFSET);
                    let y_offset = rng.gen_range(-POSITION_MAX_OFFSET..POSITION_MAX_OFFSET);

                    let position = Vec2::new(
                        ((x - x_min) as f32 - (x_max - x_min) as f32 / 2.) * SOLAR_SYSTEMS_SPACING
                            + x_offset,
                        ((y - y_min) as f32 - (y_max - y_min) as f32 / 2.) * SOLAR_SYSTEMS_SPACING
                            + y_offset,
                    );

                    let mut rng = StdRng::seed_from_u64(seed);
                    build_star(c, &mut rng, position);

                    if x == solar_system.x() && y == solar_system.y() {
                        c.spawn((
                            Name::new("UniverseMapCamera"),
                            UniverseMapCamera,
                            Camera2d,
                            Camera {
                                order: 1,
                                ..default()
                            },
                            OrthographicProjection {
                                scale: BASE_SCALE,
                                near: -1000.0,
                                far: 1000.0,
                                ..OrthographicProjection::default_2d()
                            },
                            Transform::from_translation(position.extend(0.)),
                        ));
                    }
                }
            }
        });
}

pub fn clean_universe_map(
    mut commands: Commands,
    q_universe_map: Query<Entity, With<UniverseMap>>,
    mut q_main_camera: Query<&mut Camera, With<MainCamera>>,
    mut q_solar_systems: Query<&mut Visibility, With<SolarSystem>>,
) {
    if let Ok(entity) = q_universe_map.get_single() {
        commands.entity(entity).despawn_recursive();
    }

    if let Ok(mut solar_system_visibility) = q_solar_systems.get_single_mut() {
        *solar_system_visibility = Visibility::Inherited;
    }

    if let Ok(mut camera) = q_main_camera.get_single_mut() {
        camera.is_active = true;
    }
}

pub fn update_universe_map(
    time: Res<Time>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut ev_motion: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_camera: Query<(&mut Transform, &mut OrthographicProjection), With<UniverseMapCamera>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some((mut transform, mut projection)) = q_camera.iter_mut().next() else {
        return;
    };

    for scroll in ev_scroll.read() {
        projection.scale *= 1. - ZOOM_SPEED * scroll.y * time.delta_secs();
    }

    if projection.scale < SWITCH_TO_SOLAR_SYSTEM {
        next_state.set(GameState::GameSolarSystem);
    }

    let mut camera_delta = Vec2::ZERO;

    if mouse_button_input.pressed(MouseButton::Left) {
        for motion in ev_motion.read() {
            let mut delta = motion.delta * time.delta_secs() * PAN_SPEED;
            delta.y *= -1.;
            camera_delta -= delta;
        }
    }

    if keyboard_input.any_pressed(vec![KeyCode::ArrowLeft, KeyCode::KeyA]) {
        camera_delta.x -= time.delta_secs() * PAN_KEYBOARD_SPEED;
    }

    if keyboard_input.any_pressed(vec![KeyCode::ArrowRight, KeyCode::KeyD]) {
        camera_delta.x += time.delta_secs() * PAN_KEYBOARD_SPEED;
    }

    if keyboard_input.any_pressed(vec![KeyCode::ArrowUp, KeyCode::KeyW]) {
        camera_delta.y += time.delta_secs() * PAN_KEYBOARD_SPEED;
    }

    if keyboard_input.any_pressed(vec![KeyCode::ArrowDown, KeyCode::KeyS]) {
        camera_delta.y -= time.delta_secs() * PAN_KEYBOARD_SPEED;
    }

    camera_delta *= SOLAR_SYSTEMS_SPACING;
    transform.translation.x += camera_delta.x;
    transform.translation.y += camera_delta.y;
}
