use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    items::{CanCraftResult, Inventory, LogisticRequest, Recipe, RECIPES},
    ui::{spawn_inventory_ui, HudButtonAction, HudButtonBundle, UIWindow, UIWindowParent},
};

#[derive(Bundle)]
pub struct CrafterBundle {
    crafter: Crafter,
    inventory: Inventory, //FIXME: crafting can be blocked if inventory is full of requested items. Solution: input inventory + output inventory ?
    pointer_event: On<Pointer<Click>>,
    pickable: PickableBundle,
}

impl CrafterBundle {
    pub fn new(possible_recipes: &'static [&'static str]) -> Self {
        Self {
            crafter: Crafter::new(possible_recipes),
            inventory: Inventory::new(100),
            pointer_event: On::<Pointer<Click>>::run(spawn_window),
            pickable: PickableBundle::default(),
        }
    }
}

#[derive(Component)]
pub struct Crafter {
    recipe: Option<CrafterRecipe>,
    possible_recipes: &'static [&'static str],
    cooldown: Timer,
}

impl Crafter {
    pub fn new(possible_recipes: &'static [&'static str]) -> Self {
        Self {
            recipe: None,
            possible_recipes,
            cooldown: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }

    pub fn set_recipe(&mut self, recipe: Recipe) {
        self.recipe = Some(CrafterRecipe::new(recipe));
    }
}

pub struct CrafterRecipe {
    recipe: Recipe,
    progress: Timer,
}

impl CrafterRecipe {
    pub fn new(recipe: Recipe) -> Self {
        Self {
            progress: Timer::from_seconds(recipe.time(), TimerMode::Once),
            recipe,
        }
    }
}

fn spawn_window(
    mut commands: Commands,
    listener: Listener<Pointer<Click>>,
    q_ui_window_parent: Query<Entity, With<UIWindowParent>>,
    q_crafter: Query<(&Crafter, &Inventory)>,
) {
    let parent = q_ui_window_parent.single();

    let entity = listener.listener();
    let (crafter, inventory) = q_crafter.get(entity).unwrap();

    commands
        .entity(parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(UIWindow::default()).with_children(|c| {
                // List recipes
                for recipe in crafter.possible_recipes {
                    let recipe = RECIPES.get(recipe).unwrap();

                    c.spawn(HudButtonBundle::new(HudButtonAction::SetCrafterRecipe(
                        entity, *recipe,
                    )))
                    .with_children(|c| {
                        c.spawn(TextBundle::from_section(
                            recipe.text(),
                            TextStyle {
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ));
                    });
                }

                // Inventory
                spawn_inventory_ui(c, inventory);
            });
        });
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
                            commands
                                .entity(entity)
                                .insert(LogisticRequest::new(missing_inputs));
                        }
                    }
                }

                //TODO
                CanCraftResult::NotEnoughSpace => {}
            }
        }
    }
}
