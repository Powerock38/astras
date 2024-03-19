use bevy::utils::HashMap;

use super::ElementOnAstre;

#[derive(Debug)]
pub struct Inventory {
    items: HashMap<&'static str, u32>, // Item ID -> Quantity
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
        }
    }

    fn add(&mut self, id: &'static str, quantity: u32) {
        if let Some(item) = self.items.get_mut(id) {
            *item += quantity;
        } else {
            self.items.insert(id, quantity);
        }
    }

    pub fn transfer_to(&mut self, other: &mut Inventory, id: &'static str, quantity: u32) {
        if let Some(item) = self.items.get_mut(id) {
            if *item >= quantity {
                *item -= quantity;
                other.add(id, quantity);
            }
        }
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

        Self { items }
    }
}
