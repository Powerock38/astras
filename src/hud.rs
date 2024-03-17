use bevy::prelude::*;

use crate::{PlacingBuilding, BUILDINGS};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const HUD_BUTTONS: &[HudButtonData] = &[
    HudButtonData {
        action: |commands| commands.insert_resource(PlacingBuilding(BUILDINGS[0])),
        text: "Spawn Quarry",
    },
    HudButtonData {
        action: |commands| commands.insert_resource(PlacingBuilding(BUILDINGS[1])),
        text: "Spawn Cargo Stop",
    },
];

#[derive(Clone, Copy)]
struct HudButtonData {
    pub action: fn(&mut Commands),
    pub text: &'static str,
}

#[derive(Component)]
pub struct HudButton(HudButtonData);

pub fn update_hud(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &HudButton,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (hud_button, interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                (hud_button.0.action)(&mut commands);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = hud_button.0.text.to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn setup_hud(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Buttons

            for hud_button_data in HUD_BUTTONS {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(30.0),
                            border: UiRect::all(Val::Px(2.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(HudButton(*hud_button_data))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ));
                    });
            }
        });
}
