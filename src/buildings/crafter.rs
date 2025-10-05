use bevy::prelude::*;

use crate::{
    SpriteLoader,
    buildings::BuildingHighlight,
    data::RecipeId,
    items::{CanCraftResult, Inventory, LogisticRequest, LogisticScope},
};

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
#[require(Inventory)] //FIXME: crafting can be blocked if inventory is full of requested items (and recipe outputs more than inputs)
pub struct Crafter {
    recipe: Option<CrafterRecipe>,
    possible_recipes: Vec<RecipeId>,
    cooldown: Timer,
    is_construction_site: bool,
}

impl Crafter {
    fn new(possible_recipes: Vec<RecipeId>, is_construction_site: bool) -> Self {
        Self {
            recipe: if possible_recipes.len() == 1 {
                Some(CrafterRecipe::new(possible_recipes[0]))
            } else {
                None
            },
            possible_recipes,
            cooldown: Timer::from_seconds(1.0, TimerMode::Repeating),
            is_construction_site,
        }
    }

    pub fn new_construction_site(possible_recipes: Vec<RecipeId>) -> Self {
        Self::new(possible_recipes, true)
    }

    pub fn new_crafter(possible_recipes: Vec<RecipeId>) -> Self {
        Self::new(possible_recipes, false)
    }

    pub fn set_recipe(&mut self, recipe: RecipeId) {
        self.recipe = Some(CrafterRecipe::new(recipe));
    }

    pub fn possible_recipes(&self) -> &Vec<RecipeId> {
        &self.possible_recipes
    }

    pub fn is_construction_site(&self) -> bool {
        self.is_construction_site
    }
}

#[derive(Reflect, Default)]
pub struct CrafterRecipe {
    recipe: RecipeId,
    progress: Timer,
}

impl CrafterRecipe {
    pub fn new(recipe: RecipeId) -> Self {
        let duration = recipe.data().time();
        Self {
            progress: Timer::from_seconds(duration, TimerMode::Once),
            recipe,
        }
    }
}

pub fn update_crafters(
    mut commands: Commands,
    time: Res<Time>,
    mut q_crafters: Query<(
        Entity,
        &mut Crafter,
        &mut Inventory,
        Option<&mut LogisticRequest>,
        &Transform,
        &ChildOf,
    )>,
) {
    for (entity, mut crafter, mut inventory, logistic_request, transform, child_of) in
        &mut q_crafters
    {
        // If a recipe is selected
        if let Some(recipe_crafter) = &mut crafter.recipe {
            // Try crafting
            match inventory.can_craft(recipe_crafter.recipe) {
                // Craft
                CanCraftResult::Yes => {
                    commands.entity(entity).remove::<LogisticRequest>();

                    if recipe_crafter.progress.tick(time.delta()).is_finished() {
                        recipe_crafter.progress.reset();
                        let building_output = inventory.craft(recipe_crafter.recipe);

                        // SPAWN BUILDING if output is a building
                        if let Some(building) = building_output.map(|b| b.data()) {
                            debug!("Crafted building: {}", building.name);

                            if crafter.is_construction_site {
                                commands.entity(entity).despawn();
                            }

                            commands.entity(child_of.parent()).with_children(|c| {
                                let mut ec = c.spawn((
                                    BuildingHighlight,
                                    SpriteLoader {
                                        texture_path: building.sprite_path(),
                                        ..default()
                                    },
                                    *transform,
                                ));

                                (building.on_build)(&mut ec);
                            });
                        }
                    }
                }

                // Request missing inputs
                CanCraftResult::MissingInputs(missing_inputs) => {
                    if let Some(mut logistic_request) = logistic_request {
                        if logistic_request.items() != &missing_inputs {
                            debug!("Changed missing inputs: {missing_inputs:?}");
                            logistic_request.set_items(missing_inputs);
                        }
                    } else {
                        debug!("New missing inputs: {missing_inputs:?}");
                        commands
                            .entity(entity)
                            .insert(LogisticRequest::new(missing_inputs, LogisticScope::Planet));
                    }
                }

                CanCraftResult::NotEnoughSpace => {
                    commands.entity(entity).remove::<LogisticRequest>();
                }
            }
        }
    }
}
