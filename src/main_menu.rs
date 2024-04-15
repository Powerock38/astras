use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use rand::{distributions::Alphanumeric, Rng};

use crate::{universe::spawn_solar_system, GameState, SaveGame, SaveName};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct MainMenu;

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((MainMenu, Camera2dBundle::default()));

    commands
        .spawn((
            MainMenu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    On::<Pointer<Click>>::run(spawn_new_game),
                    PickableBundle::default(),
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "New game",
                        TextStyle {
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn update_main_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
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

pub fn spawn_new_game(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    println!("New game");

    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    commands.insert_resource(SaveName(rand_string.clone()));

    spawn_solar_system(&mut commands);

    let filename = format!("{rand_string}-0");

    commands.insert_resource(SaveGame(filename.clone()));

    next_state.set(GameState::Game);
}

pub fn clean_main_menu(mut commands: Commands, q: Query<Entity, With<MainMenu>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
