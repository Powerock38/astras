use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use buildings::BuildingsPlugin;
use handle_loader::*;
use items::ItemsPlugin;
use main_menu::*;
use save_load::*;
use state::*;
use ui::UIPlugin;
use universe::UniversePlugin;
use uuid::Uuid;

mod buildings;
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
        .add_plugins(DefaultPickingPlugins)
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        // .insert_resource(DebugPickingMode::Noisy)
        .insert_resource(ClearColor(Color::BLACK))
        .register_type::<SaveName>()
        .register_type::<SpriteLoader>()
        .register_type::<MeshType>()
        .register_type::<TimerMode>()
        .register_type::<Option<Uuid>>()
        .register_type::<Option<Vec3>>()
        .register_type::<Vec<String>>()
        .register_type::<[u8; 32]>()
        .add_plugins((UniversePlugin, UIPlugin, ItemsPlugin, BuildingsPlugin))
        .configure_sets(
            Update,
            (
                MainMenuSet.run_if(in_state(GameState::MainMenu)),
                GameSet.run_if(
                    in_state(GameState::GameSolarSystem)
                        .or_else(in_state(GameState::GameUniverseMap)),
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
