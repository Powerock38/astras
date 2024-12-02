use bevy::prelude::*;
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
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        // .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .register_type::<SaveName>()
        .register_type::<SpriteLoader>()
        .add_plugins((UniversePlugin, UIPlugin, ItemsPlugin, BuildingsPlugin))
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
        .add_systems(OnExit(GameState::MainMenu), clean_main_menu)
        .add_systems(
            Update,
            (
                load_solar_system,
                finish_load_solar_system.after(load_solar_system),
                (save_solar_system, save_key_shortcut, scan_sprite_loaders).in_set(GameSet),
            ),
        )
        .init_state::<GameState>()
        .run();
}
