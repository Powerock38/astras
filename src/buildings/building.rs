use bevy::{ecs::system::EntityCommands, prelude::*, window::PrimaryWindow};

use crate::{items::Inventory, DockableOnAstre, ElementExtractor, ElementExtractorBundle};

pub const BUILDINGS: &[BuildingData] = &[
    BuildingData {
        name: "Quarry",
        sprite_name: "quarry",
        location: PlacingLocation::Surface,
        build_time_seconds: 3.,
        on_build: |c| {
            c.insert(ElementExtractorBundle {
                element_extractor: ElementExtractor::new_solid(),
                inventory: Inventory::new(100),
            });
        },
    },
    BuildingData {
        name: "Cargo Stop",
        sprite_name: "cargo-stop",
        location: PlacingLocation::Orbit,
        build_time_seconds: 2.,
        on_build: |_| {},
    },
];

#[derive(Resource, Debug)]
pub struct PlacingBuilding(pub BuildingData);

#[derive(Clone, Copy, Debug)]
pub struct BuildingData {
    pub name: &'static str,
    pub sprite_name: &'static str,
    pub location: PlacingLocation,
    pub build_time_seconds: f32,
    pub on_build: fn(&mut EntityCommands),
}

#[derive(Clone, Copy, Debug)]
pub enum PlacingLocation {
    Surface,
    Orbit,
    SurfaceOrbit,
}

impl Default for PlacingLocation {
    fn default() -> Self {
        Self::SurfaceOrbit
    }
}

#[derive(Component)]
pub struct ConstructingBuilding {
    pub building: BuildingData,
    pub build_timer: Timer,
}

#[derive(Component)]
pub struct BuildingPreview;

#[derive(Component)]
pub struct Building;

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
            let world_position = world_position.extend(1.);

            // Building Preview
            if let Some((building_preview_entity, mut building_preview_transform)) =
                q_building_preview.iter_mut().next()
            {
                // there is already a building preview, update its position
                *building_preview_transform = Transform::from_translation(world_position);

                let left = mouse_input.just_pressed(MouseButton::Left);
                let right = mouse_input.just_pressed(MouseButton::Right);

                // Place building
                if left {
                    // recycle the building preview entity to keep sprite texture
                    commands
                        .entity(building_preview_entity)
                        .retain::<SpriteBundle>()
                        .insert((
                            ConstructingBuilding {
                                building: placing_building.0,
                                build_timer: Timer::from_seconds(
                                    placing_building.0.build_time_seconds,
                                    TimerMode::Once,
                                ),
                            },
                            DockableOnAstre::instant_location(placing_building.0.location),
                        ));

                    *building_preview_transform = Transform::from_translation(world_position);

                    commands.remove_resource::<PlacingBuilding>();
                }

                // Cancel placing building
                if right {
                    commands.entity(building_preview_entity).despawn();
                    commands.remove_resource::<PlacingBuilding>();
                }
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
        }
    }
}

pub fn constructing_building(
    mut commands: Commands,
    time: Res<Time>,
    mut q_building: Query<(Entity, &mut ConstructingBuilding)>,
) {
    for (entity, mut constructing_building) in q_building.iter_mut() {
        // Tick build timer
        constructing_building.build_timer.tick(time.delta());
        if constructing_building.build_timer.finished() {
            // Spawn building: recycle the ConstructingBuilding entity to keep parent, position and sprite texture
            let mut ec = commands.entity(entity);

            ec.retain::<(Parent, SpriteBundle)>()
                .insert((Sprite::default(), Building));

            (constructing_building.building.on_build)(&mut ec);
        }
    }
}
