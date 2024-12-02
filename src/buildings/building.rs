use bevy::{ecs::system::EntityCommands, prelude::*, window::PrimaryWindow};

use crate::{
    buildings::Crafter,
    data::{BuildingId, RecipeId},
    items::{Inventory, RecipeOutputs},
    universe::{Asteroid, Astre, DockableOnAstre, SHIP_Z},
    HandleLoaderBundle, SpriteLoader,
};

const BUILDING_PREVIEW_Z: f32 = SHIP_Z - 1.0;
const BUILDING_SCALE: f32 = 3.0;
const PLACING_ZONES_COLOR: Color = Color::srgba(0.5, 0.8, 0.8, 0.5);

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
    placing_building: Option<Res<PlacingBuilding>>,
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

        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
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
                    if let Some(recipe_id) = RecipeId::ALL.iter().find(|recipe_id| match recipe_id
                        .data()
                        .outputs()
                    {
                        RecipeOutputs::Building(building_id) => building_id == placing_building.0,
                        _ => false,
                    }) {
                        let recipe_needed_space = recipe_id.data().inputs_quantity();

                        // spawn the building at building_preview_transform
                        commands.spawn((
                            HandleLoaderBundle {
                                loader: SpriteLoader {
                                    texture_path: format!("sprites/{}.png", building.sprite_name),
                                    ..default()
                                },
                                transform: *building_preview_transform,
                                ..default()
                            },
                            ConstructionSite {
                                building: placing_building.0,
                            },
                            DockableOnAstre::instant_location(building.location),
                            Crafter::new(vec![*recipe_id], true),
                            Inventory::new(recipe_needed_space),
                        ));

                        commands.entity(building_preview_entity).despawn_recursive();

                        commands.remove_resource::<PlacingBuilding>();
                    } else {
                        println!("WARNING: Building {:?} has no recipe", placing_building.0);
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

pub fn draw_placing_zones(
    mut gizmos: Gizmos,
    placing_building: Option<Res<PlacingBuilding>>,
    q_astre: Query<(&Astre, &GlobalTransform), Without<Asteroid>>,
) {
    if let Some(placing_building) = placing_building {
        let location = placing_building.0.data().location;

        for (astre, global_transform) in q_astre.iter() {
            if matches!(
                location,
                PlacingLocation::Surface | PlacingLocation::SurfaceOrAtmosphere
            ) {
                gizmos.circle_2d(
                    global_transform.translation().truncate(),
                    astre.surface_radius(),
                    PLACING_ZONES_COLOR,
                );
            }

            if matches!(
                location,
                PlacingLocation::Atmosphere | PlacingLocation::SurfaceOrAtmosphere
            ) {
                gizmos.circle_2d(
                    global_transform.translation().truncate(),
                    astre.surface_radius() + astre.atmosphere_radius(),
                    PLACING_ZONES_COLOR,
                );
            }
        }
    }
}
