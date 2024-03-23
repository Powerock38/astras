use bevy::prelude::*;

use crate::items::Inventory;

#[derive(Bundle)]
pub struct WarehouseBundle {
    pub warehouse: Warehouse,
    pub inventory: Inventory,
}

impl Default for WarehouseBundle {
    fn default() -> Self {
        Self {
            warehouse: Warehouse,
            inventory: Inventory::new(100_000),
        }
    }
}

#[derive(Component)]
pub struct Warehouse;
