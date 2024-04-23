use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::Crafter,
    items::RECIPES,
    ui::{spawn_inventory_ui, HudWindow, HudWindowParent, UiButtonBundle},
};

pub fn scan_crafter_ui(
    mut commands: Commands,
    q_crafter: Query<(Entity, &Crafter), Added<Crafter>>,
) {
    for (entity, crafter) in q_crafter.iter() {
        if !crafter.is_construction_site() {
            commands
                .entity(entity)
                .insert(On::<Pointer<Click>>::run(spawn_crafter_ui));
        }
    }
}

pub fn spawn_crafter_ui(
    mut commands: Commands,
    listener: Listener<Pointer<Click>>,
    q_window_parent: Query<Entity, With<HudWindowParent>>,
    q_crafter: Query<&Crafter>,
) {
    let parent = q_window_parent.single();
    let entity = listener.listener();
    let crafter = q_crafter.get(entity).unwrap();

    commands
        .entity(parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(HudWindow::default()).with_children(|c| {
                // List recipes
                for recipe in crafter.possible_recipes() {
                    let callback = {
                        let recipe = recipe.clone();
                        move |mut q_crafter: Query<&mut Crafter>| {
                            let mut crafter = q_crafter.get_mut(entity).unwrap();
                            crafter.set_recipe(recipe.clone());
                        }
                    };

                    c.spawn(UiButtonBundle::new(On::<Pointer<Click>>::run(callback)))
                        .with_children(|c| {
                            c.spawn(TextBundle::from_section(
                                RECIPES
                                    .get(recipe)
                                    .map(|r| r.text())
                                    .unwrap_or(recipe.clone()),
                                TextStyle {
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });
                }

                // Inventory
                spawn_inventory_ui(c, entity);
            });
        });
}
