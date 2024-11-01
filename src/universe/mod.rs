use bevy::{prelude::*, sprite::Material2dPlugin, transform::TransformSystem};

use crate::{register_material, GameState, SolarSystemSet, UniverseMapSet};

mod universe_map;
pub use universe_map::*;

mod solar_system;
pub use solar_system::*;

mod camera;
pub use camera::*;

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

mod orbit;
pub use orbit::*;

mod worm;
pub use worm::*;

mod laser;
pub use laser::*;

mod asteroid;
pub use asteroid::*;

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
            .register_type::<SolarSystem>()
            .register_type::<Ship>()
            .register_type::<Astre>()
            .register_type::<Planet>()
            .register_type::<Star>()
            .register_type::<Asteroid>()
            .register_type::<Laser>()
            .register_type::<PlanetColors>()
            .register_type::<DockableOnAstre>()
            .register_type::<Orbit>()
            .register_type::<Worm>()
            .register_type::<WormSegment>()
            .add_systems(OnEnter(GameState::GameUniverseMap), spawn_universe_map)
            .add_systems(OnExit(GameState::GameUniverseMap), clean_universe_map)
            .add_systems(
                Update,
                (
                    (
                        spawn_camera,
                        update_camera,
                        spawn_ship_sprite,
                        scan_astres,
                        update_orbits,
                        update_ship,
                        update_planets,
                        update_worms,
                        update_lasers,
                        update_asteroids,
                    )
                        .in_set(SolarSystemSet),
                    (update_universe_map,).in_set(UniverseMapSet),
                ),
            )
            .add_systems(
                PostUpdate,
                update_dockable_on_astre
                    .after(TransformSystem::TransformPropagate)
                    .in_set(SolarSystemSet),
            );

        register_material!(app, PlanetMaterial);
        register_material!(app, StarMaterial);
        register_material!(app, AsteroidMaterial);
        register_material!(app, LaserMaterial);
    }
}
