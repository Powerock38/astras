use bevy::prelude::*;

use crate::items::{Inventory, LogisticProvider, LogisticScope};

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(Inventory, LogisticProvider::new(LogisticScope::Planet))]
pub struct Warehouse;
