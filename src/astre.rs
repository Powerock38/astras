use bevy::prelude::*;

use crate::items::ElementOnAstre;

#[derive(Component)]
pub struct Astre {
    pub composition: Vec<ElementOnAstre>,
    pub temperature: u32, // in Kelvin  // TODO: why
}
