use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    input::mouse::MouseWheel,
    prelude::*,
};

use crate::{
    background::{spawn_background, Background, BackgroundMaterial},
    Ship,
};

const CAMERA_DOLLY_MAX_LENGTH: f32 = 0.05;

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

        spawn_background(c, meshes, background_materials);
    });
}

pub fn update_camera(
    time: Res<Time>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut background_query: Query<&mut Transform, With<Background>>,
    ship_query: Query<&Ship>,
    window: Query<&Window>,
) {
    let window = window.single();

    if let Some(mut projection) = query.iter_mut().next() {
        for scroll in scroll_evr.read() {
            projection.scale *= 1. - 2. * scroll.y * time.delta_seconds();
        }

        for mut transform in background_query.iter_mut() {
            transform.scale = Vec3::new(window.width(), window.height(), 0.0)
                * projection.scale
                * (1. + 2. * CAMERA_DOLLY_MAX_LENGTH);
        }

        for ship in ship_query.iter() {
            let dolly_offset = (ship.speed() * 0.00001).clamp_length_max(CAMERA_DOLLY_MAX_LENGTH);
            projection.viewport_origin = dolly_offset + Vec2::new(0.5, 0.5);
        }
    }
}
