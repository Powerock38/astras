use bevy::{prelude::*, sprite::Material2dPlugin, transform::TransformSystem};
use bevy_mod_picking::prelude::*;

use astres::AstresPlugin;
use background::BackgroundMaterial;
use buildings::BuildingsPlugin;
use dockable_on_astre::*;
use ship::*;
use solar_system::*;
use ui::UIPlugin;
use worm::*;

mod astres;
mod background;
mod buildings;
mod constants;
mod dockable_on_astre;
mod items;
mod ship;
mod solar_system;
mod ui;
mod worm;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(DebugPickingMode::Normal)
        .add_plugins((BuildingsPlugin, AstresPlugin, UIPlugin))
        .add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
        .add_systems(Startup, spawn_solar_system)
        .add_systems(Update, (update_worms, update_ship, update_camera))
        .add_systems(
            PostUpdate,
            update_dockable_on_astre.after(TransformSystem::TransformPropagate),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
