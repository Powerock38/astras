use std::time::Duration;

use astre::*;
use background::*;
use bevy::{
    asset::ChangeWatcher, prelude::*, sprite::Material2dPlugin, transform::TransformSystem,
};
use dexterous_developer::{
    dexterous_developer_setup, hot_bevy_main, InitialPlugins, ReloadableApp, ReloadableAppContents,
    ReloadableElementsSetup,
};
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
        .add_plugins(
            initial_plugins
                .initialize::<DefaultPlugins>()
                .set(AssetPlugin {
                    watch_for_changes: Some(ChangeWatcher {
                        delay: Duration::from_secs(1),
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins((Material2dPlugin::<PlanetMaterial>::default(), Material2dPlugin::<BackgroundMaterial>::default()))
        .add_systems(Startup, spawn_solar_system)
        .add_plugins(AstrasPlugin)
        .run();
}
