use bevy::{prelude::*, sprite::Material2dPlugin, transform::TransformSystem};
use bevy_mod_picking::prelude::*;

use astres::AstresPlugin;
use background::BackgroundMaterial;
use buildings::BuildingsPlugin;
use dockable_on_astre::*;
use items::ItemsPlugin;
use save::*;
use ship::*;
use solar_system::*;
use ui::UIPlugin;
use worm::*;

mod astres;
mod background;
mod buildings;
mod dockable_on_astre;
mod items;
mod save;
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
        .add_plugins((BuildingsPlugin, AstresPlugin, UIPlugin, ItemsPlugin))
        .add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
        .register_type::<DockableOnAstre>()
        .register_type::<Ship>()
        .register_type::<SolarSystem>()
        .register_type::<Worm>()
        .register_type::<WormSegment>()
        .register_type::<&'static str>()
        .register_type_data::<&'static str, ReflectSerialize>()
        // .register_type_data::<&'static str, ReflectDeserialize>() //TODO: replace all &'static str with String ?
        .add_systems(PreStartup, spawn_solar_system)
        .add_systems(
            Update,
            (update_worms, update_ship, update_camera, save_solar_system, load_scene_system),
        )
        .add_systems(
            PostUpdate,
            update_dockable_on_astre.after(TransformSystem::TransformPropagate),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
