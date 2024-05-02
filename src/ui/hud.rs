use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::{PlacingBuilding, BUILDINGS},
    ui::{spawn_inventory_ui, UiButtonBundle},
    universe::Ship,
    MainCamera,
};

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct HudWindowParent;

#[derive(Component)]
pub struct HudWindowDependent;

#[derive(Bundle)]
pub struct HudWindow {
    node: NodeBundle,
}

impl Default for HudWindow {
    fn default() -> Self {
        Self {
            node: NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: Color::rgba(0.1, 0.1, 0.1, 0.5).into(),
                ..default()
            },
        }
    }
}

pub fn setup_hud(mut commands: Commands, q_camera: Query<Entity, Added<MainCamera>>) {
    let Some(camera) = q_camera.iter().next() else {
        return;
    };

    commands
        .spawn((
            Hud,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(5.0),
                    ..default()
                },
                ..default()
            },
            Pickable::IGNORE,
            TargetCamera(camera),
        ))
        .with_children(|c| {
            // Toolbar
            c.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(5.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                Pickable::IGNORE,
            ))
            .with_children(|c| {
                for (&building_id, building) in BUILDINGS.entries() {
                    let callback = {
                        move |_event: &mut ListenerInput<Pointer<Click>>,
                              target_commands: &mut EntityCommands| {
                            target_commands
                                .commands()
                                .insert_resource(PlacingBuilding(building_id.to_string()));
                        }
                    };

                    c.spawn(UiButtonBundle::new(
                        On::<Pointer<Click>>::target_commands_mut(callback),
                    ))
                    .with_children(|c| {
                        c.spawn(TextBundle::from_section(
                            building.name,
                            TextStyle {
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ));
                    });
                }
            });

            // Windows
            c.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(80.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                },
                HudWindowParent,
                Pickable::IGNORE,
            ));
        });
}

pub fn clear_ui_or_spawn_ship_ui(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_window_parent: Query<Entity, With<HudWindowParent>>,
    q_window_dependent: Query<Entity, With<HudWindowDependent>>,
    q_children: Query<Entity, With<Children>>,
    q_ship: Query<Entity, With<Ship>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Escape, KeyCode::KeyE]) {
        let parent = q_window_parent.single();

        if q_children.get(parent).is_ok() {
            commands.entity(parent).despawn_descendants();

            for entity in q_window_dependent.iter() {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            // SPAWN SHIP UI

            let Some(entity) = q_ship.iter().next() else {
                return;
            };

            commands.entity(parent).with_children(|c| {
                c.spawn(HudWindow::default()).with_children(|c| {
                    spawn_inventory_ui(c, entity);
                });
            });
        }
    }
}
