use bevy::{prelude::*, sprite::Material2dPlugin, transform::TransformSystem, utils::Uuid};
use bevy_mod_picking::prelude::*;

use astres::{AstresPlugin, PlanetMaterial, StarMaterial};
use background::BackgroundMaterial;
use buildings::BuildingsPlugin;
use camera::*;
use dockable_on_astre::*;
use handle_loader::*;
use items::ItemsPlugin;
use save::*;
use ship::*;
use solar_system::*;
use ui::UIPlugin;
use worm::*;

mod astres;
mod background;
mod buildings;
mod camera;
mod dockable_on_astre;
mod handle_loader;
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
        .register_type::<SpriteLoader>()
        .register_type::<DockableOnAstre>()
        .register_type::<Ship>()
        .register_type::<SolarSystem>()
        .register_type::<Worm>()
        .register_type::<WormSegment>()
        .register_type::<TimerMode>()
        .register_type::<Option<Uuid>>()
        .register_type::<Option<Vec3>>()
        .register_type::<Vec<String>>()
        .add_systems(PreStartup, spawn_solar_system)
        .add_systems(
            Update,
            (
                spawn_camera,
                spawn_ship_sprite,
                scan_sprite_loaders,
                scan_atres_material_loaders::<PlanetMaterial>,
                scan_atres_material_loaders::<StarMaterial>,
                update_worms,
                update_ship,
                update_camera,
                save_solar_system,
                load_scene_system,
            ),
        )
        .add_systems(
            PostUpdate,
            update_dockable_on_astre.after(TransformSystem::TransformPropagate),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
