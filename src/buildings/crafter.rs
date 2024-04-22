use bevy::prelude::*;

use crate::items::{CanCraftResult, Inventory, LogisticRequest, LogisticScope, RECIPES};

#[derive(Bundle)]
pub struct CrafterBundle {
    crafter: Crafter,
    inventory: Inventory, //FIXME: crafting can be blocked if inventory is full of requested items. Solution: input inventory + output inventory ?
}

impl CrafterBundle {
    pub fn new(possible_recipes: &'static [&'static str]) -> Self {
        Self {
            crafter: Crafter::new(possible_recipes),
            inventory: Inventory::new(100),
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Crafter {
    recipe: Option<CrafterRecipe>,
    possible_recipes: Vec<String>,
    cooldown: Timer,
}

impl Crafter {
    pub fn new(possible_recipes: &'static [&'static str]) -> Self {
        Self {
            recipe: if possible_recipes.len() == 1 {
                Some(CrafterRecipe::new(possible_recipes[0].to_string()))
            } else {
                None
            },
            possible_recipes: possible_recipes
                .to_vec()
                .iter()
                .map(|s| s.to_string())
                .collect(),
            cooldown: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }

    pub fn set_recipe(&mut self, recipe: String) {
        self.recipe = Some(CrafterRecipe::new(recipe));
    }

    pub fn possible_recipes(&self) -> &Vec<String> {
        &self.possible_recipes
    }
}

#[derive(Reflect, Default)]
pub struct CrafterRecipe {
    recipe: String,
    progress: Timer,
}

impl CrafterRecipe {
    pub fn new(recipe: String) -> Self {
        Self {
            progress: Timer::from_seconds(RECIPES[&recipe].time(), TimerMode::Once),
            recipe,
        }
    }
}

pub fn update_crafters(
    mut commands: Commands,
    time: Res<Time>,
    mut q_crafter: Query<(
        Entity,
        &mut Crafter,
        &mut Inventory,
        Option<&mut LogisticRequest>,
    )>,
) {
    for (entity, mut crafter, mut inventory, logistic_request) in q_crafter.iter_mut() {
        // If a recipe is selected
        if let Some(recipe_crafter) = &mut crafter.recipe {
            // Try crafting
            match inventory.can_craft(&recipe_crafter.recipe) {
                // Craft
                CanCraftResult::Yes => {
                    commands.entity(entity).remove::<LogisticRequest>();

                    if recipe_crafter.progress.tick(time.delta()).just_finished() {
                        inventory.craft(&recipe_crafter.recipe);
                        recipe_crafter.progress.reset();
                    }
                }

                // Request missing inputs
                CanCraftResult::MissingInputs(missing_inputs) => {
                    if crafter.cooldown.tick(time.delta()).finished() {
                        if let Some(mut logistic_request) = logistic_request {
                            if logistic_request.items() != &missing_inputs {
                                println!("Changed missing inputs: {:?}", missing_inputs);
                                logistic_request.set_items(missing_inputs);
                            }
                        } else {
                            println!("New missing inputs: {:?}", missing_inputs);
                            commands.entity(entity).insert(LogisticRequest::new(
                                missing_inputs,
                                LogisticScope::Planet,
                            ));
                        }
                    }
                }

                CanCraftResult::NotEnoughSpace => {
                    commands.entity(entity).remove::<LogisticRequest>();
                }
            }
        }
    }
}
