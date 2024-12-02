use bevy::prelude::*;

use crate::items::Inventory;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Inventory)]
pub struct Spaceport;
