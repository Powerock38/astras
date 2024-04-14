use bevy::{ecs::system::EntityCommands, prelude::*, window::PrimaryWindow};

use crate::{
    buildings::{
        CrafterBundle, ElementExtractorBundle, LogisticFreightBundle, SpaceportBundle,
        WarehouseBundle,
    },
    universe::{DockableOnAstre, SHIP_Z},
    HandleLoaderBundle, SpriteLoader,
};

pub static BUILDINGS: phf::Map<&'static str, BuildingData> = phf::phf_map! {
    "quarry" => BuildingData {
        name: "Quarry",
        sprite_name: "quarry",
        location: PlacingLocation::Surface,
        build_time_seconds: 3.,
        on_build: |c| {
            c.insert(ElementExtractorBundle::new_solid());
        },
    },
    "liquid_extractor" => BuildingData {
        name: "Liquid Extractor",
        sprite_name: "quarry",
        location: PlacingLocation::Surface,
        build_time_seconds: 3.,
        on_build: |c| {
            c.insert(ElementExtractorBundle::new_liquid());
        },
    },
    "atmosphere_harvester" => BuildingData {
        name: "Atmosphere Harvester",
        sprite_name: "quarry",
        location: PlacingLocation::Atmosphere,
        build_time_seconds: 3.,
        on_build: |c| {
            c.insert(ElementExtractorBundle::new_gas());
        },
    },
    "plasma_catalyser" => BuildingData {
        name: "Plasma Catalyser",
        sprite_name: "quarry",
        location: PlacingLocation::SurfaceOrAtmosphere,
        build_time_seconds: 3.,
        on_build: |c| {
            c.insert(ElementExtractorBundle::new_plasma());
        },
    },
    "warehouse" => BuildingData {
        name: "Warehouse",
        sprite_name: "warehouse",
        location: PlacingLocation::Surface,
        build_time_seconds: 2.,
        on_build: |c| {
            c.insert(WarehouseBundle::default());
        },
    },
    "cargo_shuttle" => BuildingData {
        name: "Cargo Shuttle",
        sprite_name: "ship",
        location: PlacingLocation::SurfaceOrAtmosphere,
        build_time_seconds: 1.,
        on_build: |c| {
            c.insert(LogisticFreightBundle::new_planet());
        },
    },
    "spaceport" => BuildingData {
        name: "Spaceport",
        sprite_name: "warehouse",
        location: PlacingLocation::Atmosphere,
        build_time_seconds: 1.,
        on_build: |c| {
            c.insert(SpaceportBundle::default());
        },
    },
    "interplanetary_freighter" => BuildingData {
        name: "Interplanetary Freighter",
        sprite_name: "ship",
        location: PlacingLocation::Atmosphere,
        build_time_seconds: 1.,
        on_build: |c| {
            c.insert(LogisticFreightBundle::new_solar_system());
        },
    },
    "smelter" => BuildingData {
        name: "Smelter",
        sprite_name: "smelter",
        location: PlacingLocation::Surface,
        build_time_seconds: 3.,
        on_build: |c| {
            c.insert(CrafterBundle::new(&[
                "smelt_electronite_ore"
            ]));
        },
    },
};

const BUILDING_PREVIEW_Z: f32 = SHIP_Z - 1.0;

#[derive(Resource, Debug)]
pub struct PlacingBuilding(pub String);

#[derive(Clone, Copy, Debug)]
pub struct BuildingData {
    pub name: &'static str,
    pub sprite_name: &'static str,
    pub location: PlacingLocation,
    pub build_time_seconds: f32,
    pub on_build: fn(&mut EntityCommands),
}

#[derive(Clone, Copy, Reflect, Default, Debug)]
pub enum PlacingLocation {
    Surface,
    Atmosphere,
    #[default]
    SurfaceOrAtmosphere,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ConstructingBuilding {
    pub building: String,
    pub build_timer: Timer,
}

#[derive(Component)]
pub struct BuildingPreview;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Building;

pub fn place_building(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    placing_building: Option<ResMut<PlacingBuilding>>,
    mut q_building_preview: Query<(Entity, &mut Transform), With<BuildingPreview>>,
) {
    // Resource PlacingBuilding stores the building that is currently being placed
    if let Some(placing_building) = placing_building {
        let Some((camera, camera_transform)) = q_camera.iter().next() else {
            return;
        };

        let Some(cursor_position) = q_windows.single().cursor_position() else {
            return;
        };

        if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position)
        {
            let world_position = world_position.extend(BUILDING_PREVIEW_Z);

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
                        .retain::<(SpriteBundle, SpriteLoader)>()
                        .insert((
                            ConstructingBuilding {
                                building: placing_building.0.clone(),
                                build_timer: Timer::from_seconds(
                                    BUILDINGS[&placing_building.0].build_time_seconds,
                                    TimerMode::Once,
                                ),
                            },
                            DockableOnAstre::instant_location(
                                BUILDINGS[&placing_building.0].location,
                            ),
                        ));

                    commands.remove_resource::<PlacingBuilding>();
                }

                // Cancel placing building
                if right {
                    commands.entity(building_preview_entity).despawn();
                    commands.remove_resource::<PlacingBuilding>();
                }
            } else {
                // there is no building preview, spawn it
                let transform = Transform::from_translation(world_position);

                commands.spawn((
                    HandleLoaderBundle {
                        loader: SpriteLoader {
                            texture_path: format!(
                                "sprites/{}.png",
                                BUILDINGS[&placing_building.0].sprite_name
                            ),
                            color: Color::rgba(1., 1., 1., 0.5),
                        },
                        transform,
                        ..default()
                    },
                    BuildingPreview,
                ));
            }
        }
    }
}

pub fn constructing_building(
    mut commands: Commands,
    time: Res<Time>,
    mut q_building: Query<(Entity, &mut ConstructingBuilding, &mut SpriteLoader)>,
) {
    for (entity, mut constructing_building, mut sprite_loader) in q_building.iter_mut() {
        // Tick build timer
        constructing_building.build_timer.tick(time.delta());
        if constructing_building.build_timer.finished() {
            // Spawn building: recycle the ConstructingBuilding entity to keep parent, position and sprite texture

            sprite_loader.color = Color::default();

            let mut ec = commands.entity(entity);

            ec.retain::<(Parent, SpriteBundle, SpriteLoader)>()
                .insert((Sprite::default(), Building));

            (BUILDINGS[&constructing_building.building].on_build)(&mut ec);
        }
    }
}
