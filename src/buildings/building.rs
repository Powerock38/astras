use bevy::{ecs::system::EntityCommands, prelude::*, window::PrimaryWindow};

use crate::{
    buildings::Crafter,
    data::{BuildingId, RecipeId},
    items::{Inventory, RecipeOutputs},
    universe::{Asteroid, Astre, DockableOnAstre, SHIP_Z},
    SpriteLoader,
};

const BUILDING_PREVIEW_Z: f32 = SHIP_Z - 1.0;
const BUILDING_SCALE: f32 = 3.0;
const PLACING_ZONES_COLOR: Color = Color::srgba(0.5, 0.8, 0.8, 0.5);
const HIGHLIGHT_COLOR: Color = Color::srgb(0.0, 1.0, 1.0);

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
    CloseOrbit,
}

#[derive(Component)]
pub struct BuildingPreview;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BuildingHighlight;

pub fn spawn_building(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window, With<PrimaryWindow>>,
    placing_building: Res<PlacingBuilding>,
    q_building_preview: Option<Single<(Entity, &mut Transform), With<BuildingPreview>>>,
) {
    // Resource PlacingBuilding stores the building that is currently being placed
    let (camera, camera_transform) = *q_camera;

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let building = placing_building.0.data();

    if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
        let world_position = world_position.extend(BUILDING_PREVIEW_Z);

        // Building Preview
        if let Some((building_preview_entity, mut building_preview_transform)) =
            q_building_preview.map(Single::into_inner)
        {
            // there is already a building preview, update its position
            building_preview_transform.translation = world_position;

            let left = mouse_input.just_pressed(MouseButton::Left);
            let right = mouse_input.just_pressed(MouseButton::Right);

            // Place construction site
            if left {
                if let Some(recipe_id) =
                    RecipeId::ALL
                        .iter()
                        .find(|recipe_id| match recipe_id.data().outputs() {
                            RecipeOutputs::Building(building_id) => {
                                building_id == placing_building.0
                            }
                            _ => false,
                        })
                {
                    let recipe_needed_space = recipe_id.data().inputs_quantity();

                    // spawn the construction site at building_preview_transform
                    commands.spawn((
                        SpriteLoader {
                            texture_path: format!("sprites/{}.png", building.sprite_name),
                            color: Color::default().with_alpha(0.8),
                        },
                        *building_preview_transform,
                        BuildingHighlight,
                        DockableOnAstre::instant_location(building.location),
                        Crafter::new_construction_site(vec![*recipe_id]),
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
                commands.entity(building_preview_entity).despawn_recursive();
                commands.remove_resource::<PlacingBuilding>();
            }
        } else {
            // there is no building preview, spawn it
            commands.spawn((
                SpriteLoader {
                    texture_path: format!("sprites/{}.png", building.sprite_name),
                    color: Color::default().with_alpha(0.5),
                },
                Transform::from_translation(world_position).with_scale(Vec3::splat(BUILDING_SCALE)),
                BuildingPreview,
            ));
        }
    }
}

pub fn draw_placing_zones(
    mut gizmos: Gizmos,
    placing_building: Res<PlacingBuilding>,
    q_astres: Query<(&Astre, &GlobalTransform, &InheritedVisibility), Without<Asteroid>>,
) {
    let location = placing_building.0.data().location;

    for (astre, global_transform, _) in q_astres.iter().filter(|(_, _, v)| v.get()) {
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

        if astre.has_atmosphere()
            && matches!(
                location,
                PlacingLocation::Atmosphere | PlacingLocation::SurfaceOrAtmosphere
            )
        {
            gizmos.circle_2d(
                global_transform.translation().truncate(),
                astre.atmosphere_radius(),
                PLACING_ZONES_COLOR,
            );
        }

        if matches!(location, PlacingLocation::CloseOrbit) {
            gizmos.circle_2d(
                global_transform.translation().truncate(),
                astre.close_orbit_radius(),
                PLACING_ZONES_COLOR,
            );
        }
    }
}

pub fn add_highlight_selection(
    mut commands: Commands,
    query: Query<Entity, Added<BuildingHighlight>>,
) {
    for entity in &query {
        commands
            .entity(entity)
            .observe(recolor_on::<Pointer<Over>>(HIGHLIGHT_COLOR))
            .observe(recolor_on::<Pointer<Out>>(Color::default()));
    }
}

fn recolor_on<E>(color: Color) -> impl Fn(Trigger<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.entity()) else {
            return;
        };
        sprite.color = color.with_alpha(sprite.color.alpha());
    }
}
