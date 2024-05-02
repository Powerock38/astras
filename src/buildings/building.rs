use bevy::{ecs::system::EntityCommands, prelude::*, window::PrimaryWindow};

use crate::{
    buildings::{
        Crafter, CrafterBundle, ExtractorBundle, LogisticFreightBundle, SpaceportBundle,
        WarehouseBundle,
    },
    items::{Inventory, Recipe, RECIPES},
    universe::{DockableOnAstre, SHIP_Z},
    HandleLoaderBundle, SpriteLoader,
};

pub static BUILDINGS: phf::Map<&'static str, BuildingData> = phf::phf_map! {
    "quarry" => BuildingData {
        name: "Quarry",
        sprite_name: "quarry",
        location: PlacingLocation::Surface,
        scale: BUILDING_SCALE,
        on_build: |c| {
            c.insert(ExtractorBundle::new_solid());
        },
    },
    "liquid_extractor" => BuildingData {
        name: "Liquid Extractor",
        sprite_name: "quarry",
        location: PlacingLocation::Surface,
        scale: BUILDING_SCALE,
        on_build: |c| {
            c.insert(ExtractorBundle::new_liquid());
        },
    },
    "atmosphere_harvester" => BuildingData {
        name: "Atmosphere Harvester",
        sprite_name: "quarry",
        location: PlacingLocation::Atmosphere,
        scale: BUILDING_SCALE,
        on_build: |c| {
            c.insert(ExtractorBundle::new_gas());
        },
    },
    "plasma_catalyser" => BuildingData {
        name: "Plasma Catalyser",
        sprite_name: "quarry",
        location: PlacingLocation::SurfaceOrAtmosphere,
        scale: BUILDING_SCALE,
        on_build: |c| {
            c.insert(ExtractorBundle::new_plasma());
        },
    },
    "warehouse" => BuildingData {
        name: "Warehouse",
        sprite_name: "warehouse",
        location: PlacingLocation::Surface,
        scale: BUILDING_SCALE,
        on_build: |c| {
            c.insert(WarehouseBundle::default());
        },
    },
    "cargo_shuttle" => BuildingData {
        name: "Cargo Shuttle",
        sprite_name: "cargo_shuttle",
        location: PlacingLocation::SurfaceOrAtmosphere,
        scale: BUILDING_SCALE / 2.0,
        on_build: |c| {
            c.insert(LogisticFreightBundle::new_planet());
        },
    },
    "spaceport" => BuildingData {
        name: "Spaceport",
        sprite_name: "spaceport",
        location: PlacingLocation::Atmosphere,
        scale: BUILDING_SCALE,
        on_build: |c| {
            c.insert(SpaceportBundle::default());
        },
    },
    "interplanetary_freighter" => BuildingData {
        name: "Interplanetary Freighter",
        sprite_name: "cargo_shuttle",
        location: PlacingLocation::Atmosphere,
        scale: BUILDING_SCALE,
        on_build: |c| {
            c.insert(LogisticFreightBundle::new_solar_system());
        },
    },
    "foundry" => BuildingData {
        name: "Foundry",
        sprite_name: "foundry",
        location: PlacingLocation::Surface,
        scale: BUILDING_SCALE,
        on_build: |c| {
            c.insert(CrafterBundle::new(vec![
                "smelt_electronite_ore".to_string(),
                "craft_plasma_fuel".to_string(),
            ]));
        },
    },
    "assembler" => BuildingData {
        name: "Assembler",
        sprite_name: "assembler",
        location: PlacingLocation::Surface,
        scale: BUILDING_SCALE,
        on_build: |c| {
            c.insert(CrafterBundle::new(vec![
                "craft_computing_core".to_string(),
                "spawn_cargo_shuttle".to_string(),
            ]));
        },
    },
};

const BUILDING_PREVIEW_Z: f32 = SHIP_Z - 1.0;
const BUILDING_SCALE: f32 = 3.0;

#[derive(Resource, Debug)]
pub struct PlacingBuilding(pub String);

#[derive(Clone, Copy, Debug)]
pub struct BuildingData {
    pub name: &'static str,
    pub sprite_name: &'static str,
    pub location: PlacingLocation,
    pub on_build: fn(&mut EntityCommands),
    pub scale: f32,
}

impl BuildingData {
    #[inline]
    pub fn sprite_path(&self) -> String {
        format!("sprites/{}.png", self.sprite_name)
    }
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
pub struct ConstructionSite {
    pub building: String,
}

#[derive(Component)]
pub struct BuildingPreview;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Building;

pub fn spawn_building(
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
                building_preview_transform.translation = world_position;

                let left = mouse_input.just_pressed(MouseButton::Left);
                let right = mouse_input.just_pressed(MouseButton::Right);

                // Place building
                if left {
                    let recipe_needed_space = RECIPES
                        .get(&placing_building.0)
                        .map_or(0, Recipe::inputs_quantity);

                    // recycle the building preview entity to keep sprite texture
                    commands
                        .entity(building_preview_entity)
                        .retain::<(SpriteBundle, SpriteLoader)>()
                        .insert((
                            ConstructionSite {
                                building: placing_building.0.clone(),
                            },
                            DockableOnAstre::instant_location(
                                BUILDINGS[&placing_building.0].location,
                            ),
                            Crafter::new(vec![placing_building.0.clone()], true),
                            Inventory::new(recipe_needed_space),
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
                let transform = Transform::from_translation(world_position)
                    .with_scale(Vec3::splat(BUILDINGS[&placing_building.0].scale));

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
