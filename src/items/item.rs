use bevy::platform::collections::HashMap;

use crate::data::ItemId;

pub struct Item {
    pub name: &'static str,
    pub description: &'static str,
}

pub type ItemMap = HashMap<ItemId, u32>; // Item ID -> Quantity
