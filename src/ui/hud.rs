use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::{PlacingBuilding, BUILDINGS},
    ui::UiButtonBundle,
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
                    ..default()
                },
                background_color: Color::rgb(0.10, 0.10, 0.10).into(),
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
                for (building_id, building) in BUILDINGS.entries() {
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

pub fn remove_windows_on_escape(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_window_parent: Query<Entity, With<HudWindowParent>>,
    q_window_dependent: Query<Entity, With<HudWindowDependent>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        for entity in q_window_dependent.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let parent = q_window_parent.single();
        commands.entity(parent).despawn_descendants();
    }
}
