use bevy::prelude::*;
use bevy_mod_picking::picking_core::Pickable;

use crate::{
    buildings::{Crafter, PlacingBuilding, BUILDINGS},
    LoadGame,
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub enum HudButtonAction {
    LoadGame(String),
    SetPlacingBuilding(&'static str),
    SetCrafterRecipe(Entity, &'static str),
}

#[derive(Bundle)]
pub struct HudButtonBundle {
    button: ButtonBundle,
    action: HudButtonAction,
}

impl HudButtonBundle {
    pub fn new(action: HudButtonAction) -> Self {
        Self {
            button: ButtonBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(30.0),
                    border: UiRect::all(Val::Px(2.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            action,
        }
    }
}

#[derive(Component)]
pub struct UIWindowParent;

#[derive(Component)]
pub struct UIWindowDependent;

#[derive(Bundle)]
pub struct UIWindow {
    node: NodeBundle,
}

impl Default for UIWindow {
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

pub fn update_hud(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &HudButtonAction,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    // Queries for HudButtonAction
    mut q_crafter: Query<&mut Crafter>,
) {
    for (action, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                match action {
                    HudButtonAction::SetPlacingBuilding(building) => {
                        commands.insert_resource(PlacingBuilding(building));
                    }
                    HudButtonAction::SetCrafterRecipe(crafter_entity, recipe) => {
                        let mut crafter = q_crafter.get_mut(*crafter_entity).unwrap();
                        crafter.set_recipe(*recipe);
                        border_color.0 = Color::WHITE;
                    }
                    HudButtonAction::LoadGame(file) => {
                        commands.insert_resource(LoadGame(file.clone()));
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn setup_hud(mut commands: Commands, q_camera: Query<Entity, With<MainCamera>>) {
    commands
        .spawn((
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
            TargetCamera(q_camera.single()),
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
                    c.spawn(HudButtonBundle::new(HudButtonAction::SetPlacingBuilding(
                        building_id,
                    )))
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
                UIWindowParent,
                Pickable::IGNORE,
            ));
        });
}

pub fn remove_windows_on_escape(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_ui_window_parent: Query<Entity, With<UIWindowParent>>,
    q_ui_window_dependent: Query<Entity, With<UIWindowDependent>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        for entity in q_ui_window_dependent.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let parent = q_ui_window_parent.single();
        commands.entity(parent).despawn_descendants();
    }
}
