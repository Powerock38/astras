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

const BASE_SCALE: f32 = 100.0;
const PAN_SPEED: f32 = 0.2;
const ZOOM_SPEED: f32 = 2.0;
const SWITCH_TO_SOLAR_SYSTEM: f32 = 50.0;

#[derive(Component)]
pub struct UniverseMapDependent;

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
            UniverseMapDependent,
            SpatialBundle {
                transform: Transform::from_scale(Vec3::splat(0.1)),
                ..default()
            },
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
                            UniverseMapDependent,
                            Camera2dBundle {
                                camera: Camera {
                                    order: 1,
                                    ..default()
                                },
                                projection: OrthographicProjection {
                                    scale: BASE_SCALE,
                                    near: -1000.0,
                                    far: 1000.0,
                                    ..default()
                                },
                                transform: Transform::from_translation(position.extend(0.)),
                                ..default()
                            },
                        ));
                    }
                }
            }
        });
}

pub fn clean_universe_map(
    mut commands: Commands,
    q_universe_map: Query<Entity, With<UniverseMapDependent>>,
    mut q_main_camera: Query<&mut Camera, With<MainCamera>>,
    mut q_solar_systems: Query<&mut Visibility, With<SolarSystem>>,
) {
    for entity in q_universe_map.iter() {
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
    mut q_projection: Query<&mut OrthographicProjection, With<UniverseMapCamera>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(mut projection) = q_projection.iter_mut().next() else {
        return;
    };

    for scroll in ev_scroll.read() {
        projection.scale *= 1. - ZOOM_SPEED * scroll.y * time.delta_seconds();
    }

    if projection.scale < SWITCH_TO_SOLAR_SYSTEM {
        next_state.set(GameState::GameSolarSystem);
    }

    //TODO: improve + support keyboard
    for motion in ev_motion.read() {
        if mouse_button_input.pressed(MouseButton::Left) {
            let mut delta = motion.delta.normalize() * time.delta_seconds() * PAN_SPEED;
            delta.y *= -1.;
            projection.viewport_origin += delta;
        }
    }
}
