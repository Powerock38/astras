use bevy::{prelude::*, sprite::Material2dPlugin, transform::TransformSystem};

use crate::{scan_atres_material_loaders, MaterialLoader};

mod solar_system;
pub use solar_system::*;

mod background;
pub use background::*;

mod ship;
pub use ship::*;

mod astre;
pub use astre::*;

mod planet;
pub use planet::*;

mod star;
pub use star::*;

mod dockable_on_astre;
pub use dockable_on_astre::*;

mod worm;
pub use worm::*;

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            Material2dPlugin::<BackgroundMaterial>::default(),
            Material2dPlugin::<StarMaterial>::default(),
            Material2dPlugin::<PlanetMaterial>::default(),
        ))
        .register_type::<SolarSystem>()
        .register_type::<Ship>()
        .register_type::<Astre>()
        .register_type::<Planet>()
        .register_type::<Star>()
        .register_type::<PlanetMaterial>()
        .register_type::<PlanetColors>()
        .register_type::<StarMaterial>()
        .register_type::<MaterialLoader<PlanetMaterial>>()
        .register_type::<MaterialLoader<StarMaterial>>()
        .register_type::<DockableOnAstre>()
        .register_type::<Worm>()
        .register_type::<WormSegment>()
        .add_systems(PreStartup, spawn_solar_system)
        .add_systems(
            Update,
            (
                spawn_ship_sprite,
                scan_atres_material_loaders::<PlanetMaterial>,
                scan_atres_material_loaders::<StarMaterial>,
                update_ship,
                update_planets,
                update_worms,
            ),
        )
        .add_systems(
            PostUpdate,
            update_dockable_on_astre.after(TransformSystem::TransformPropagate),
        );
    }
}
