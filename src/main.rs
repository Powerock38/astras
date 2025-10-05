use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use buildings::BuildingsPlugin;
use handle_loader::*;
use main_menu::*;
use save_load::*;
use state::*;
use ui::UIPlugin;
use universe::UniversePlugin;

mod buildings;
mod data;
mod enum_map;
mod handle_loader;
mod items;
mod main_menu;
mod save_load;
mod state;
mod ui;
mod universe;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            // bevy::diagnostic::LogDiagnosticsPlugin::default(),
            // bevy::diagnostic::FrameTimeDiagnosticsPlugin
            // RemotePlugin::default(),
            // RemoteHttpPlugin::default().with_header("Access-Control-Allow-Origin", "*"),
        ))
        .add_plugins((UniversePlugin, UIPlugin, BuildingsPlugin))
        .insert_resource(ClearColor(Color::BLACK))
        .configure_sets(
            PreUpdate,
            (SolarSystemSet.run_if(in_state(GameState::GameSolarSystem)),),
        )
        .configure_sets(
            Update,
            (
                MainMenuSet.run_if(in_state(GameState::MainMenu)),
                GameSet.run_if(
                    in_state(GameState::GameSolarSystem).or(in_state(GameState::GameUniverseMap)),
                ),
                SolarSystemSet.run_if(in_state(GameState::GameSolarSystem)),
                UniverseMapSet.run_if(in_state(GameState::GameUniverseMap)),
            ),
        )
        .configure_sets(
            PostUpdate,
            (SolarSystemSet.run_if(in_state(GameState::GameSolarSystem)),),
        )
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(
            Update,
            ((
                scan_sprite_loaders,
                (|mut commands: Commands| {
                    commands.queue(SaveUniverse);
                })
                .run_if(input_just_pressed(KeyCode::KeyL)),
            )
                .in_set(GameSet),),
        )
        .add_observer(load_universe)
        .init_state::<GameState>()
        .run();
}
