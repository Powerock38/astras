use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::Crafter,
    items::{Inventory, RECIPES},
    ui::{spawn_inventory_ui, HudButtonAction, HudButtonBundle, UIWindow, UIWindowParent},
};

pub fn spawn_crafter_ui(
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
                for recipe in crafter.possible_recipes() {
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
