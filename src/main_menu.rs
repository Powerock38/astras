use bevy::prelude::*;
use rand::Rng;

use crate::{
    GameState, UniverseName,
    ui::{UiButton, build_load_ui},
    universe::{build_ship, build_solar_system},
};

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((DespawnOnExit(GameState::MainMenu), Camera2d));

    commands
        .spawn((
            DespawnOnExit(GameState::MainMenu),
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
    _pointer_click: On<Pointer<Click>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("New game");

    let mut rng = rand::rng();
    let solar_system_position = [rng.random::<i32>(), rng.random::<i32>()];

    let timestamp = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
    commands.insert_resource(UniverseName(format!("universe_{timestamp}")));

    commands
        .spawn((build_solar_system(solar_system_position),))
        .with_child(build_ship());

    next_state.set(GameState::GameSolarSystem);
}
