use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use rand::{distributions::Alphanumeric, Rng};

use crate::{
    ui::{build_load_ui, UiButtonBundle},
    universe::spawn_solar_system,
    GameState, SaveName,
};

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
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|c| {
            c.spawn(UiButtonBundle::new(On::<Pointer<Click>>::run(
                spawn_new_game,
            )))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "New game",
                    TextStyle {
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });

            build_load_ui(c);
        });
}

pub fn spawn_new_game(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    println!("New game");

    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    commands.insert_resource(SaveName(rand_string));

    spawn_solar_system(&mut commands);

    //let filename = format!("{rand_string}-0");
    //commands.insert_resource(SaveGame(filename.clone()));

    next_state.set(GameState::Game);
}

pub fn clean_main_menu(mut commands: Commands, q: Query<Entity, With<MainMenu>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
