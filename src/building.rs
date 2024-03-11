use bevy::{prelude::*, window::PrimaryWindow};

use crate::DockableOnAstre;

pub const BUILDINGS: [BuildingData; 1] = [BuildingData {
    name: "Quarry",
    sprite_name: "quarry",
}];

#[derive(Clone, Copy, Debug)]
pub struct BuildingData {
    pub name: &'static str,
    pub sprite_name: &'static str,
}

#[derive(Resource, Debug)]
pub struct PlacingBuilding(pub BuildingData);

#[derive(Component)]
pub struct BuildingPreview;

#[derive(Component)]
pub struct Building(BuildingData);

pub fn place_building(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    placing_building: Option<ResMut<PlacingBuilding>>,
    mut q_building_preview: Query<(Entity, &mut Transform), With<BuildingPreview>>,
    asset_server: Res<AssetServer>,
) {
    // Resource PlacingBuilding stores the building that is currently being placed
    if let Some(placing_building) = placing_building {
        // Get cursor world position
        let (camera, camera_transform) = q_camera.single();
        let window = q_windows.single();
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let world_position = world_position.extend(100.);

            // Building Preview
            if let Some((_, mut transform)) = q_building_preview.iter_mut().next() {
                // there is already a building preview, update its position
                *transform = Transform::from_translation(world_position);
            } else {
                // there is no building preview, spawn it
                let texture =
                    asset_server.load(format!("sprites/{}.png", placing_building.0.sprite_name));
                let transform = Transform::from_translation(world_position);

                commands
                    .spawn(SpriteBundle {
                        texture,
                        transform,
                        sprite: Sprite {
                            color: Color::rgba(1., 1., 1., 0.5),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(BuildingPreview);
            }

            let left = mouse_input.just_pressed(MouseButton::Left);
            let right = mouse_input.just_pressed(MouseButton::Right);

            // Place building
            if left {
                let transform = Transform::from_translation(world_position);

                commands
                    .spawn(SpriteBundle {
                        texture: asset_server
                            .load(format!("sprites/{}.png", placing_building.0.sprite_name)),
                        transform,
                        ..default()
                    })
                    .insert((Building(placing_building.0), DockableOnAstre::forever()));
            }

            // Stop placing building
            if left || right {
                for (entity, _) in q_building_preview.iter_mut() {
                    commands.entity(entity).despawn();
                }
                commands.remove_resource::<PlacingBuilding>();
            }
        }
    }
}
