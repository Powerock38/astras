use bevy::prelude::*;
use rand::Rng;

use crate::{
    ui::{build_load_ui, UiButton},
    universe::spawn_solar_system,
    GameState, UniverseName,
};

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((StateScoped(GameState::MainMenu), Camera2d));

    commands
        .spawn((
            StateScoped(GameState::MainMenu),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|c| {
            c.spawn(UiButton)
                .observe(spawn_new_game)
                .with_children(|parent| {
                    parent.spawn(Text::new("New game"));
                });

            build_load_ui(c);
        });
}

fn spawn_new_game(
    _trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("New game");

    let solar_system_position = [
        rand::thread_rng().gen::<i32>(),
        rand::thread_rng().gen::<i32>(),
    ];

    let timestamp = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    commands.insert_resource(UniverseName(format!("universe_{timestamp}")));

    spawn_solar_system(&mut commands, solar_system_position);

    next_state.set(GameState::GameSolarSystem);
}
