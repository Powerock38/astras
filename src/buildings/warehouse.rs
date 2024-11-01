use bevy::prelude::*;

use crate::items::{Inventory, LogisticProvider, LogisticScope};

#[derive(Bundle)]
pub struct WarehouseBundle {
    pub warehouse: Warehouse,
    pub inventory: Inventory,
    pub logistic_provider: LogisticProvider,
}

impl Default for WarehouseBundle {
    fn default() -> Self {
        Self {
            warehouse: Warehouse,
            inventory: Inventory::new(100_000),
            logistic_provider: LogisticProvider::new(LogisticScope::Planet),
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Warehouse;
