use bevy::{prelude::*, sprite::Material2dPlugin, transform::TransformSystem};

use astre::*;
use background::*;
use building::*;
use dockable_on_astre::*;
use hud::*;
use ship::*;
use solar_system::*;
use worm::*;

mod astre;
mod background;
mod building;
mod constants;
mod dockable_on_astre;
mod hud;
mod ship;
mod solar_system;
mod worm;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            Material2dPlugin::<PlanetMaterial>::default(),
            Material2dPlugin::<BackgroundMaterial>::default(),
        ))
        .add_systems(Startup, (spawn_solar_system, setup_hud))
        .add_systems(
            Update,
            (
                update_astres,
                update_worms,
                update_ship,
                update_camera,
                update_hud,
                place_building,
                constructing_building,
            ),
        )
        .add_systems(
            PostUpdate,
            update_dockable_on_astre.after(TransformSystem::TransformPropagate),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
