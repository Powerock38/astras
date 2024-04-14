use bevy::{prelude::*, sprite::Material2dPlugin};

mod astre;
pub use astre::*;

mod planet;
pub use planet::*;

mod star;
pub use star::*;

use crate::MaterialLoader;

pub struct AstresPlugin;

impl Plugin for AstresPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            Material2dPlugin::<StarMaterial>::default(),
            Material2dPlugin::<PlanetMaterial>::default(),
        ))
        .register_type::<Astre>()
        .register_type::<Planet>()
        .register_type::<Star>()
        .register_type::<PlanetMaterial>()
        .register_type::<PlanetColors>()
        .register_type::<StarMaterial>()
        .register_type::<MaterialLoader<PlanetMaterial>>()
        .register_type::<MaterialLoader<StarMaterial>>()
        .add_systems(Update, (update_planets,));
    }
}
