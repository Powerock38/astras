use astre::*;
use bevy::{prelude::*, transform::TransformSystem};
use dockable_on_astre::*;
use marker::*;
use ship::*;
use solar_system::*;
use utils::reparent_system;
use worm::*;

mod astre;
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
        .add_systems(Startup, setup_universe)
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
        .run();
}

fn setup_universe(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_solar_system(commands, meshes, materials)
}
