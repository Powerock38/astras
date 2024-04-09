use bevy::prelude::*;

use crate::items::Inventory;

#[derive(Bundle)]
pub struct SpaceportBundle {
    spaceport: Spaceport,
    inventory: Inventory,
}

impl Default for SpaceportBundle {
    fn default() -> Self {
        Self {
            spaceport: Spaceport,
            inventory: Inventory::new(1000),
        }
    }
}

#[derive(Component)]
pub struct Spaceport;
