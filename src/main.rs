use astre::*;
use background::*;
use bevy::{prelude::*, sprite::Material2dPlugin, transform::TransformSystem};
use dockable_on_astre::*;
use marker::*;
use ship::*;
use solar_system::*;
use utils::reparent_system;
use worm::*;

mod astre;
mod background;
mod constants;
mod dockable_on_astre;
mod marker;
mod ship;
mod solar_system;
mod utils;
mod worm;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            Material2dPlugin::<PlanetMaterial>::default(),
            Material2dPlugin::<BackgroundMaterial>::default(),
        ))
        .add_systems(Startup, spawn_solar_system)
        .add_systems(
            Update,
            (
                update_astres,
                update_worms,
                update_ship,
                update_camera,
                update_marker,
                reparent_system,
            ),
        )
        .add_systems(
            PostUpdate,
            update_dockable_on_astre.after(TransformSystem::TransformPropagate),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
