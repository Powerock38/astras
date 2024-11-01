use bevy::prelude::*;

use crate::{
    buildings::{Building, BUILDINGS},
    items::{CanCraftResult, Inventory, LogisticRequest, LogisticScope, Recipe, RECIPES},
    HandleLoaderBundle, SpriteLoader,
};

#[derive(Bundle)]
pub struct CrafterBundle {
    crafter: Crafter,
    inventory: Inventory, //FIXME: crafting can be blocked if inventory is full of requested items (and recipe outputs more than inputs)
}

impl CrafterBundle {
    pub fn new(possible_recipes: Vec<String>) -> Self {
        Self {
            crafter: Crafter::new(possible_recipes, false),
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
    is_construction_site: bool,
}

impl Crafter {
    pub fn new(possible_recipes: Vec<String>, is_construction_site: bool) -> Self {
        Self {
            recipe: if possible_recipes.len() == 1 {
                Some(CrafterRecipe::new(possible_recipes[0].clone()))
            } else {
                None
            },
            possible_recipes: possible_recipes.iter().map(ToString::to_string).collect(),
            cooldown: Timer::from_seconds(1.0, TimerMode::Repeating),
            is_construction_site,
        }
    }

    pub fn set_recipe(&mut self, recipe: String) {
        self.recipe = Some(CrafterRecipe::new(recipe));
    }

    pub fn possible_recipes(&self) -> &Vec<String> {
        &self.possible_recipes
    }

    #[inline]
    pub fn is_construction_site(&self) -> bool {
        self.is_construction_site
    }
}

#[derive(Reflect, Default)]
pub struct CrafterRecipe {
    recipe: String,
    progress: Timer,
}

impl CrafterRecipe {
    pub fn new(recipe: String) -> Self {
        let duration = RECIPES.get(&recipe).map_or(1.0, Recipe::time);
        Self {
            progress: Timer::from_seconds(duration, TimerMode::Once),
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
        &Transform,
        &Parent,
    )>,
) {
    for (entity, mut crafter, mut inventory, logistic_request, transform, parent) in &mut q_crafter
    {
        // If a recipe is selected
        if let Some(recipe_crafter) = &mut crafter.recipe {
            // Try crafting
            match inventory.can_craft(&recipe_crafter.recipe) {
                // Craft
                CanCraftResult::Yes => {
                    commands.entity(entity).remove::<LogisticRequest>();

                    if recipe_crafter.progress.tick(time.delta()).finished() {
                        recipe_crafter.progress.reset();
                        let building_output = inventory.craft(&recipe_crafter.recipe);

                        // SPAWN BUILDING if output is a building
                        if let Some(building) = building_output.and_then(|b| BUILDINGS.get(&b)) {
                            println!("Crafted building: {}", building.name);

                            if crafter.is_construction_site {
                                commands.entity(entity).despawn_recursive();
                            }

                            commands.entity(parent.get()).with_children(|c| {
                                let mut ec = c.spawn((
                                    Building,
                                    HandleLoaderBundle {
                                        loader: SpriteLoader {
                                            texture_path: building.sprite_path(),
                                            ..default()
                                        },
                                        transform: *transform,
                                        ..default()
                                    },
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
                            println!("Changed missing inputs: {missing_inputs:?}");
                            logistic_request.set_items(missing_inputs);
                        }
                    } else {
                        println!("New missing inputs: {missing_inputs:?}");
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
