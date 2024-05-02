use bevy::prelude::*;

use crate::items::{ElementOnAstre, ItemMap, RecipeOutputs, RECIPES};

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

    fn add(&mut self, id: &String, quantity: u32) {
        if let Some(item) = self.items.get_mut(id) {
            *item += quantity;
        } else {
            self.items.insert(id.clone(), quantity);
        }
    }

    fn remove(&mut self, id: &String, quantity: u32) {
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
    pub fn transfer_to(&mut self, other: &mut Inventory, id: String, max_quantity: u32) -> u32 {
        if let Some(item_quantity) = self.items.get_mut(&id) {
            // Adjust quantity if self doesn't have enough quantity
            let mut real_quantity = (*item_quantity).min(max_quantity);

            // If other's size is not infinite
            if other.size != 0 {
                // Adjust quantity if other doesn't have enough space
                real_quantity = other.remaining_space().min(real_quantity);
            }

            if real_quantity > 0 {
                self.remove(&id, real_quantity);
                other.add(&id, real_quantity);

                return real_quantity;
            }
        }

        0
    }

    pub fn can_craft(&self, recipe: &String) -> CanCraftResult {
        let Some(recipe) = RECIPES.get(recipe) else {
            return CanCraftResult::Yes;
        };

        let has_space_for_outputs = self.size == 0
            || recipe
                .outputs_quantity()
                .saturating_sub(recipe.inputs_quantity())
                <= self.remaining_space();

        if !has_space_for_outputs {
            return CanCraftResult::NotEnoughSpace;
        }

        let has_inputs = recipe
            .inputs()
            .iter()
            .all(|(id, quantity)| self.quantity(&(*id).to_string()) >= *quantity);

        if !has_inputs {
            return CanCraftResult::MissingInputs(
                recipe
                    .inputs()
                    .iter()
                    .filter_map(|(id, quantity)| {
                        if self.quantity(&(*id).to_string()) < *quantity {
                            Some((
                                (*id).to_string(),
                                quantity - self.quantity(&(*id).to_string()),
                            ))
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
        }

        CanCraftResult::Yes
    }

    // if the recipe output is a building, returns its id
    pub fn craft(&mut self, recipe: &String) -> Option<String> {
        if self.can_craft(recipe).yes() {
            let Some(recipe) = RECIPES.get(recipe) else {
                return Some(recipe.clone());
            };

            for (id, quantity) in recipe.inputs() {
                self.remove(&(*id).to_string(), *quantity);
            }

            match recipe.outputs() {
                RecipeOutputs::Items(items) => {
                    for (id, quantity) in items {
                        self.add(&(*id).to_string(), *quantity);
                    }
                }
                RecipeOutputs::Building(name) => return Some(name.to_string()),
            }
        }

        None
    }

    #[inline]
    pub fn quantity(&self, id: &String) -> u32 {
        *self.items.get(id).unwrap_or(&0)
    }

    #[inline]
    pub fn all_ids(&self) -> Vec<String> {
        self.items.keys().cloned().collect()
    }

    #[inline]
    pub fn items(&self) -> &ItemMap {
        &self.items
    }

    #[inline]
    pub fn total_quantity(&self) -> u32 {
        self.items.values().sum()
    }
}

impl From<Vec<ElementOnAstre>> for Inventory {
    fn from(elements: Vec<ElementOnAstre>) -> Self {
        let mut items = ItemMap::default();

        for element in elements {
            items.insert(element.id.to_string(), element.quantity);
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
