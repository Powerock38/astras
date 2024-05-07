use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use rand::Rng;

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

    let solar_system_position = [
        rand::thread_rng().gen::<i32>(),
        rand::thread_rng().gen::<i32>(),
    ];

    commands.insert_resource(SaveName(format!(
        "{},{}",
        solar_system_position[0], solar_system_position[1]
    )));

    spawn_solar_system(&mut commands, solar_system_position);

    next_state.set(GameState::GameSolarSystem);
}

pub fn clean_main_menu(mut commands: Commands, q: Query<Entity, With<MainMenu>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
