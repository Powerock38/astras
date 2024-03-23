use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    items::{Inventory, Recipe, RECIPES},
    HudButtonAction, HudButtonBundle, UIWindow, UIWindowParent,
};

#[derive(Bundle)]
pub struct CrafterBundle {
    crafter: Crafter,
    inventory: Inventory,
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
}

impl Crafter {
    pub fn new(possible_recipes: &'static [&'static str]) -> Self {
        Self {
            recipe: None,
            possible_recipes,
        }
    }

    pub fn set_recipe(&mut self, recipe: Recipe) {
        self.recipe = Some(CrafterRecipe::new(recipe));
    }
}

pub struct CrafterRecipe {
    recipe: Recipe,
    cooldown: Timer,
}

impl CrafterRecipe {
    pub fn new(recipe: Recipe) -> Self {
        Self {
            cooldown: Timer::from_seconds(recipe.time(), TimerMode::Once),
            recipe,
        }
    }
}

fn spawn_window(
    mut commands: Commands,
    listener: Listener<Pointer<Click>>,
    q_ui_window_parent: Query<Entity, With<UIWindowParent>>,
    q_crafter: Query<&Crafter>,
) {
    let parent = q_ui_window_parent.single();

    let entity = listener.listener();
    let crafter = q_crafter.get(entity).unwrap();

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
            });
        });
}
