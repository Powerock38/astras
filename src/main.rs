use bevy::{prelude::*, utils::Uuid};
use bevy_mod_picking::prelude::*;

use buildings::BuildingsPlugin;
use items::ItemsPlugin;
use ui::UIPlugin;
use universe::UniversePlugin;

use camera::*;
use handle_loader::*;
use main_menu::*;
use save::*;
use state::*;

mod buildings;
mod camera;
mod handle_loader;
mod items;
mod main_menu;
mod save;
mod state;
mod ui;
mod universe;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(DebugPickingMode::Normal)
        .insert_resource(ClearColor(Color::BLACK))
        .register_type::<SaveName>()
        .register_type::<SpriteLoader>()
        .register_type::<TimerMode>()
        .register_type::<Option<Uuid>>()
        .register_type::<Option<Vec3>>()
        .register_type::<Vec<String>>()
        .add_plugins((UniversePlugin, UIPlugin, ItemsPlugin, BuildingsPlugin))
        .configure_sets(
            Update,
            (
                MainMenuSet.run_if(in_state(GameState::MainMenu)),
                GameplaySet.run_if(in_state(GameState::Game)),
            ),
        )
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(OnExit(GameState::MainMenu), clean_main_menu)
        .add_systems(
            Update,
            (
                load_solar_system,
                finish_load_solar_system.after(load_solar_system),
                save_solar_system,
                (update_main_menu).in_set(MainMenuSet),
                (
                    save_key_shortcut,
                    scan_sprite_loaders,
                    spawn_camera,
                    update_camera,
                )
                    .in_set(GameplaySet),
            ),
        )
        .init_state::<GameState>()
        .run();
}
