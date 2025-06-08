use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use rand::prelude::*;

use super::ActiveSolarSystem;
use crate::{
    universe::{build_solar_system, build_star, MainCamera, Ship, SolarSystem},
    GameState, SaveUniverse,
};

const OBSERVABLE_UNIVERSE_RADIUS: i32 = 5;
const SOLAR_SYSTEMS_SPACING: f32 = 2_000_000.0;
const POSITION_MAX_OFFSET: f32 = SOLAR_SYSTEMS_SPACING * 0.5;
const SOLAR_SYSTEMS_SCALE: f32 = 0.1;
const BASE_SCALE: f32 = 600.0;
const PAN_SPEED: f32 = 0.1;
const PAN_KEYBOARD_SPEED: f32 = 1.0;
const ZOOM_SPEED: f32 = 2.0;
const SWITCH_TO_SOLAR_SYSTEM: f32 = 500.0;

#[derive(Component)]
pub struct UniverseMapCamera;

pub fn spawn_universe_map(
    mut commands: Commands,
    mut main_camera: Single<&mut Camera, With<MainCamera>>,
    q_solar_system: Single<(&SolarSystem, &mut Visibility), With<ActiveSolarSystem>>,
) {
    main_camera.is_active = false;

    let (current_solar_system, mut solar_system_visibility) = q_solar_system.into_inner();

    *solar_system_visibility = Visibility::Hidden;

    commands
        .spawn((
            Name::new("UniverseMap"),
            StateScoped(GameState::GameUniverseMap),
            Transform::from_scale(Vec3::splat(SOLAR_SYSTEMS_SCALE)),
            Visibility::default(),
        ))
        .with_children(|c| {
            let x_min = current_solar_system.x() - OBSERVABLE_UNIVERSE_RADIUS / 2;
            let x_max = current_solar_system.x() + OBSERVABLE_UNIVERSE_RADIUS / 2;
            let y_min = current_solar_system.y() - OBSERVABLE_UNIVERSE_RADIUS / 2;
            let y_max = current_solar_system.y() + OBSERVABLE_UNIVERSE_RADIUS / 2;

            for x in x_min..=x_max {
                for y in y_min..=y_max {
                    let position = [x, y];

                    let solar_system = SolarSystem { position };
                    let seed = solar_system.seed();

                    let mut rng = StdRng::seed_from_u64(seed);

                    let x_offset = rng.random_range(-POSITION_MAX_OFFSET..POSITION_MAX_OFFSET);
                    let y_offset = rng.random_range(-POSITION_MAX_OFFSET..POSITION_MAX_OFFSET);

                    let map_position = Vec2::new(
                        ((x - x_min) as f32 - (x_max - x_min) as f32 / 2.) * SOLAR_SYSTEMS_SPACING
                            + x_offset,
                        ((y - y_min) as f32 - (y_max - y_min) as f32 / 2.) * SOLAR_SYSTEMS_SPACING
                            + y_offset,
                    );

                    let mut rng = StdRng::seed_from_u64(seed);

                    c.spawn(build_star(&mut rng, map_position)).observe(
                        move |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                            commands.trigger(TravelToSolarSystem(position));
                        },
                    );

                    if x == current_solar_system.x() && y == current_solar_system.y() {
                        c.spawn((
                            Name::new("UniverseMapCamera"),
                            UniverseMapCamera,
                            Camera2d,
                            Camera {
                                order: 1,
                                ..default()
                            },
                            Projection::Orthographic(OrthographicProjection {
                                scale: BASE_SCALE,
                                near: -1000.0,
                                far: 1000.0,
                                ..OrthographicProjection::default_2d()
                            }),
                            Transform::from_translation(map_position.extend(0.)),
                        ));
                    }
                }
            }
        });
}

pub fn clean_universe_map(
    solar_system_visibility: Single<&mut Visibility, With<ActiveSolarSystem>>,
    mut main_camera: Single<&mut Camera, With<MainCamera>>,
) {
    *(solar_system_visibility.into_inner()) = Visibility::Visible;
    main_camera.is_active = true;
}

pub fn update_universe_map(
    time: Res<Time>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut ev_motion: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_camera: Single<(&mut Transform, &mut Projection), With<UniverseMapCamera>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut transform, mut projection) = q_camera.into_inner();

    let Projection::Orthographic(projection) = projection.as_mut() else {
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

#[derive(Event)]
pub struct TravelToSolarSystem(pub [i32; 2]);

pub fn travel_to_solar_system(
    trigger: Trigger<TravelToSolarSystem>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut set: ParamSet<(
        Single<(Entity, &mut Visibility), With<ActiveSolarSystem>>,
        Query<(
            Entity,
            &SolarSystem,
            &mut Visibility,
            Option<&ActiveSolarSystem>,
        )>,
    )>,
    ship_entity: Single<Entity, With<Ship>>,
) {
    info!("Travelling to solar system at {:?}", trigger.0);

    // save game just in case
    commands.queue(SaveUniverse);

    let solar_system_position = trigger.0;

    let (active_entity, mut active_solar_system_visibility) = set.p0().into_inner();
    *active_solar_system_visibility = Visibility::Hidden;
    commands.entity(active_entity).remove::<ActiveSolarSystem>();

    // un-hide new solar system, generate it if it doesn't exist
    let solar_system_entity = {
        if let Some((solar_system_entity, _, mut visibility, _)) = set
            .p1()
            .iter_mut()
            .find(|(_, s, _, _)| s.position == solar_system_position)
        {
            *visibility = Visibility::Visible;
            solar_system_entity
        } else {
            commands
                .spawn(build_solar_system(solar_system_position))
                .id()
        }
    };

    commands
        .entity(solar_system_entity)
        .insert(ActiveSolarSystem);

    commands
        .entity(*ship_entity)
        .set_parent_in_place(solar_system_entity);

    next_state.set(GameState::GameSolarSystem);
}
