use astre::*;
use bevy::{prelude::*, transform::TransformSystem};
use dexterous_developer::{hot_bevy_main, InitialPlugins, ReloadableElementsSetup, dexterous_developer_setup, ReloadableAppContents, ReloadableApp};
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

pub struct AstrasPlugin;

impl Plugin for AstrasPlugin {
    fn build(&self, app: &mut App) {
        app.setup_reloadable_elements::<reloadable>();
    }
}

#[dexterous_developer_setup]
fn reloadable(app: &mut ReloadableAppContents) {
    app
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
        );
}

#[hot_bevy_main]
fn bevy_main(initial_plugins: impl InitialPlugins) {
    App::new()
        .add_plugins(initial_plugins.initialize::<DefaultPlugins>())
        .add_systems(Startup, setup_universe)
        .add_plugins(AstrasPlugin)
        .run();
}

fn setup_universe(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_solar_system(commands, meshes, materials)
}
