use bevy::prelude::*;
use bevy::utils::HashMap;

use super::ElementOnAstre;

#[derive(Component, Debug)]
pub struct Inventory {
    items: HashMap<&'static str, u32>, // Item ID -> Quantity
    size: u32,                         // 0 = infinite
}

impl Inventory {
    pub fn new(size: u32) -> Self {
        Self {
            items: HashMap::default(),
            size,
        }
    }

    fn add(&mut self, id: &'static str, quantity: u32) {
        if let Some(item) = self.items.get_mut(id) {
            *item += quantity;
        } else {
            self.items.insert(id, quantity);
        }
    }

    fn quantity_all(&self) -> u32 {
        self.items
            .iter()
            .fold(0, |quantity, entry| quantity + entry.1)
    }

    pub fn remaining_size(&self) -> u32 {
        self.size.saturating_sub(self.quantity_all())
    }

    pub fn transfer_to(&mut self, other: &mut Inventory, id: &'static str, quantity: u32) -> u32 {
        if let Some(item_quantity) = self.items.get_mut(id) {
            // Adjust quantity if self doesn't have enough quantity
            let mut real_quantity = (*item_quantity).min(quantity);

            // If other's size is not infinite
            if other.size != 0 {
                // Adjust quantity if other doesn't have enough size
                real_quantity = (other.remaining_size().min(quantity)).min(real_quantity);
            }

            if real_quantity > 0 {
                *item_quantity -= real_quantity;
                other.add(id, quantity);

                return real_quantity;
            }
        }

        0
    }

    pub fn quantity(&self, id: &'static str) -> u32 {
        *self.items.get(id).unwrap_or(&0)
    }

    pub fn all_ids(&self) -> Vec<&'static str> {
        self.items.keys().copied().collect()
    }
}

impl From<Vec<ElementOnAstre>> for Inventory {
    fn from(elements: Vec<ElementOnAstre>) -> Self {
        let mut items = HashMap::default();

        for element in elements {
            items.insert(element.id, element.quantity);
        }

        Self { items, size: 0 }
    }
}
