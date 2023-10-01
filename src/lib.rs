use astre::*;
use bevy::{prelude::*, sprite::Material2dPlugin, transform::TransformSystem};
use dexterous_developer::{
    dexterous_developer_setup, hot_bevy_main, InitialPlugins, ReloadableApp, ReloadableAppContents,
    ReloadableElementsSetup,
};
use dockable_on_astre::*;
use marker::*;
use ship::*;
use solar_system::*;
use utils::{reparent_system, PlanetMaterial};
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
    app.add_systems(
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
        .add_plugins(Material2dPlugin::<PlanetMaterial>::default())
        .add_systems(Startup, setup_universe)
        .add_plugins(AstrasPlugin)
        .run();
}

fn setup_universe(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    color_materials: ResMut<Assets<ColorMaterial>>,
    perlin_materials: ResMut<Assets<PlanetMaterial>>,
) {
    spawn_solar_system(commands, meshes, perlin_materials, color_materials)
}
