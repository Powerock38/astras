use bevy::{
    input::common_conditions::input_just_pressed, prelude::*, sprite_render::Material2dPlugin,
};

use crate::{GameSet, GameState, SolarSystemSet, UniverseMapSet, register_material};

mod asteroid;
mod astre;
mod background;
mod camera;
mod dockable_on_astre;
mod laser;
mod orbit;
mod planet;
mod ship;
mod solar_system;
mod star;
mod universe_map;
mod worm;

pub use asteroid::*;
pub use astre::*;
pub use background::*;
pub use camera::*;
pub use dockable_on_astre::*;
pub use laser::*;
pub use orbit::*;
pub use planet::*;
pub use ship::*;
pub use solar_system::*;
pub use star::*;
pub use universe_map::*;
pub use worm::*;

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
            .add_systems(OnEnter(GameState::GameUniverseMap), spawn_universe_map)
            .add_systems(OnExit(GameState::GameUniverseMap), clean_universe_map)
            .add_systems(PreUpdate, (set_active_solar_system).in_set(SolarSystemSet))
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
                        update_planet_shadows,
                        update_worms,
                        update_lasers,
                        update_asteroids,
                        reset_camera_viewport.run_if(input_just_pressed(KeyCode::KeyR)),
                    )
                        .in_set(SolarSystemSet),
                    (update_universe_map,).in_set(UniverseMapSet),
                    ((|state: Res<State<GameState>>,
                       mut next_state: ResMut<NextState<GameState>>| {
                        match state.get() {
                            GameState::GameSolarSystem => {
                                next_state.set(GameState::GameUniverseMap);
                            }
                            GameState::GameUniverseMap => {
                                next_state.set(GameState::GameSolarSystem);
                            }
                            _ => {}
                        }
                    })
                    .run_if(input_just_pressed(KeyCode::Comma)))
                    .in_set(GameSet),
                ),
            )
            .add_systems(
                PostUpdate,
                (update_dockable_on_astre.after(TransformSystems::Propagate))
                    .in_set(SolarSystemSet),
            )
            .add_observer(travel_to_solar_system);

        register_material!(app, PlanetMaterial);
        register_material!(app, StarMaterial);
        register_material!(app, AsteroidMaterial);
        register_material!(app, LaserMaterial);
    }
}
