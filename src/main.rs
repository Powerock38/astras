use bevy::{prelude::*, utils::Uuid};
use bevy_mod_picking::prelude::*;

use buildings::BuildingsPlugin;
use items::ItemsPlugin;
use ui::UIPlugin;
use universe::UniversePlugin;

use camera::*;
use handle_loader::*;
use save::*;

mod buildings;
mod camera;
mod handle_loader;
mod items;
mod save;
mod ui;
mod universe;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(DebugPickingMode::Normal)
        .insert_resource(ClearColor(Color::BLACK))
        .register_type::<SpriteLoader>()
        .register_type::<TimerMode>()
        .register_type::<Option<Uuid>>()
        .register_type::<Option<Vec3>>()
        .register_type::<Vec<String>>()
        .add_plugins((UniversePlugin, UIPlugin, ItemsPlugin, BuildingsPlugin))
        .add_systems(
            Update,
            (
                scan_sprite_loaders,
                save_solar_system,
                load_solar_system,
                spawn_camera,
                update_camera,
            ),
        )
        .run();
}
