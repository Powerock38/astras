use bevy::prelude::*;

use crate::items::{ElementOnAstre, ItemMap, RECIPES};

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Inventory {
    items: ItemMap,
    size: u32, // 0 = infinite
}

impl Inventory {
    pub fn new(size: u32) -> Self {
        Self {
            items: ItemMap::default(),
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

    fn remove(&mut self, id: &'static str, quantity: u32) {
        if let Some(item) = self.items.get_mut(id) {
            *item -= quantity;
            if *item == 0 {
                self.items.remove(id);
            }
        }
    }

    #[inline]
    pub fn remaining_space(&self) -> u32 {
        self.size.saturating_sub(
            self.items
                .iter()
                .fold(0, |total, (_, quantity)| total + quantity),
        )
    }

    // Best-effort item transfer
    pub fn transfer_to(
        &mut self,
        other: &mut Inventory,
        id: &'static str,
        max_quantity: u32,
    ) -> u32 {
        if let Some(item_quantity) = self.items.get_mut(id) {
            // Adjust quantity if self doesn't have enough quantity
            let mut real_quantity = (*item_quantity).min(max_quantity);

            // If other's size is not infinite
            if other.size != 0 {
                // Adjust quantity if other doesn't have enough space
                real_quantity = other.remaining_space().min(real_quantity);
            }

            if real_quantity > 0 {
                self.remove(id, real_quantity);
                other.add(id, real_quantity);

                return real_quantity;
            }
        }

        0
    }

    pub fn can_craft(&self, recipe: &'static str) -> CanCraftResult {
        let recipe = RECIPES[recipe];

        let has_space_for_outputs = self.size == 0
            || recipe
                .outputs()
                .iter()
                .fold(0, |total, (_, quantity)| total + quantity)
                <= self.remaining_space();

        if !has_space_for_outputs {
            return CanCraftResult::NotEnoughSpace;
        }

        let has_inputs = recipe
            .inputs()
            .iter()
            .all(|(id, quantity)| self.quantity(id) >= *quantity);

        if !has_inputs {
            return CanCraftResult::MissingInputs(
                recipe
                    .inputs()
                    .iter()
                    .filter_map(|(id, quantity)| {
                        if self.quantity(id) < *quantity {
                            Some((*id, quantity - self.quantity(id)))
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
        }

        CanCraftResult::Yes
    }

    pub fn craft(&mut self, recipe: &'static str) {
        if self.can_craft(recipe).yes() {
            let recipe = RECIPES[recipe];

            for (id, quantity) in recipe.inputs() {
                self.remove(id, *quantity);
            }

            for (id, quantity) in recipe.outputs() {
                self.add(id, *quantity);
            }
        }
    }

    #[inline]
    pub fn quantity(&self, id: &'static str) -> u32 {
        *self.items.get(id).unwrap_or(&0)
    }

    #[inline]
    pub fn all_ids(&self) -> Vec<&'static str> {
        self.items.keys().copied().collect()
    }

    #[inline]
    pub fn items(&self) -> &ItemMap {
        &self.items
    }
}

impl From<Vec<ElementOnAstre>> for Inventory {
    fn from(elements: Vec<ElementOnAstre>) -> Self {
        let mut items = ItemMap::default();

        for element in elements {
            items.insert(element.id, element.quantity);
        }

        Self { items, size: 0 }
    }
}

pub enum CanCraftResult {
    Yes,
    NotEnoughSpace,
    MissingInputs(ItemMap),
}

impl CanCraftResult {
    pub fn yes(&self) -> bool {
        matches!(self, Self::Yes)
    }
}
