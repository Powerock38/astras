use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    remote::{http::RemoteHttpPlugin, RemotePlugin},
};
use buildings::BuildingsPlugin;
use handle_loader::*;
use items::ItemsPlugin;
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
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        // .add_plugins((bevy::diagnostic::LogDiagnosticsPlugin::default(), bevy::diagnostic::FrameTimeDiagnosticsPlugin))
        .add_plugins((
            RemotePlugin::default(),
            RemoteHttpPlugin::default().with_header("Access-Control-Allow-Origin", "*"),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .register_type::<SpriteLoader>()
        .add_plugins((UniversePlugin, UIPlugin, ItemsPlugin, BuildingsPlugin))
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
        .enable_state_scoped_entities::<GameState>()
        .run();
}
