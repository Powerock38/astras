use bevy::{ecs::system::EntityCommands, prelude::*, window::PrimaryWindow};

use crate::{
    buildings::{
        Crafter, CrafterBundle, ExtractorBundle, LogisticFreightBundle, SpaceportBundle,
        WarehouseBundle,
    },
    enum_map,
    items::{Inventory, RecipeId},
    universe::{DockableOnAstre, SHIP_Z},
    HandleLoaderBundle, SpriteLoader,
};

enum_map! {
    BuildingId => BuildingData {
        Quarry = BuildingData {
            name: "Quarry",
            sprite_name: "quarry",
            location: PlacingLocation::Surface,
            on_build: |c| {
                c.insert(ExtractorBundle::new_solid());
            },
        },

        LiquidExtractor = BuildingData {
            name: "Liquid Extractor",
            sprite_name: "quarry",
            location: PlacingLocation::Surface,
            on_build: |c| {
                c.insert(ExtractorBundle::new_liquid());
            },
        },

        AtmosphereHarvester = BuildingData {
            name: "Atmosphere Harvester",
            sprite_name: "quarry",
            location: PlacingLocation::Atmosphere,
            on_build: |c| {
                c.insert(ExtractorBundle::new_gas());
            },
        },

        PlasmaCatalyser = BuildingData {
            name: "Plasma Catalyser",
            sprite_name: "quarry",
            location: PlacingLocation::SurfaceOrAtmosphere,
            on_build: |c| {
                c.insert(ExtractorBundle::new_plasma());
            },
        },

        Warehouse = BuildingData {
            name: "Warehouse",
            sprite_name: "warehouse",
            location: PlacingLocation::Surface,
            on_build: |c| {
                c.insert(WarehouseBundle::default());
            },
        },

        CargoShuttle = BuildingData {
            name: "Cargo Shuttle",
            sprite_name: "cargo_shuttle",
            location: PlacingLocation::SurfaceOrAtmosphere,
            on_build: |c| {
                c.insert(LogisticFreightBundle::new_planet());
            },
        },

        Spaceport = BuildingData {
            name: "Spaceport",
            sprite_name: "spaceport",
            location: PlacingLocation::Atmosphere,
            on_build: |c| {
                c.insert(SpaceportBundle::default());
            },
        },

        InterplanetaryFreighter = BuildingData {
            name: "Interplanetary Freighter",
            sprite_name: "cargo_shuttle",
            location: PlacingLocation::Atmosphere,
            on_build: |c| {
                c.insert(LogisticFreightBundle::new_solar_system());
            },
        },

        Foundry = BuildingData {
            name: "Foundry",
            sprite_name: "foundry",
            location: PlacingLocation::Surface,
            on_build: |c| {
                c.insert(CrafterBundle::new(vec![
                    RecipeId::SmeltElectroniteOre,
                    RecipeId::CraftPlasmaFuel,
                ]));
            },
        },

        Assembler = BuildingData {
            name: "Assembler",
            sprite_name: "assembler",
            location: PlacingLocation::Surface,
            on_build: |c| {
                c.insert(CrafterBundle::new(vec![
                    RecipeId::CraftComputingCore,
                    RecipeId::SpawnCargoShuttle,
                ]));
            },
        },
    }
}

const BUILDING_PREVIEW_Z: f32 = SHIP_Z - 1.0;
const BUILDING_SCALE: f32 = 3.0;

#[derive(Resource, Debug)]
pub struct PlacingBuilding(pub BuildingId);

#[derive(Clone, Copy, Debug)]
pub struct BuildingData {
    pub name: &'static str,
    pub sprite_name: &'static str,
    pub location: PlacingLocation,
    pub on_build: fn(&mut EntityCommands),
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
    pub building: BuildingId,
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

        let building = placing_building.0.data();

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
                    if let Some(recipe_id) = RecipeId::from_str(placing_building.0.to_str()) {
                        let recipe_needed_space = recipe_id.data().inputs_quantity();

                        // recycle the building preview entity to keep sprite texture
                        commands
                            .entity(building_preview_entity)
                            .retain::<(SpriteBundle, SpriteLoader)>()
                            .insert((
                                ConstructionSite {
                                    building: placing_building.0,
                                },
                                DockableOnAstre::instant_location(building.location),
                                Crafter::new(vec![recipe_id], true),
                                Inventory::new(recipe_needed_space),
                            ));

                        commands.remove_resource::<PlacingBuilding>();
                    }
                }

                // Cancel placing building
                if right {
                    commands.entity(building_preview_entity).despawn();
                    commands.remove_resource::<PlacingBuilding>();
                }
            } else {
                // there is no building preview, spawn it
                let transform = Transform::from_translation(world_position)
                    .with_scale(Vec3::splat(BUILDING_SCALE));

                commands.spawn((
                    HandleLoaderBundle {
                        loader: SpriteLoader {
                            texture_path: format!("sprites/{}.png", building.sprite_name),
                            color: Color::srgba(1., 1., 1., 0.5),
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
