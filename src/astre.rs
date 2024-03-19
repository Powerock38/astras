use bevy::prelude::*;

use crate::items::Inventory;

#[derive(Component, Debug)]
pub struct Astre {
    pub inventory: Inventory,
    pub temperature: u32, // in Kelvin  // TODO: why
}
