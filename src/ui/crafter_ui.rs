use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::Crafter,
    items::{Inventory, RECIPES},
    ui::{spawn_inventory_ui, HudWindow, HudWindowParent, UiButtonBundle},
};

pub fn spawn_crafter_ui(
    mut commands: Commands,
    listener: Listener<Pointer<Click>>,
    q_window_parent: Query<Entity, With<HudWindowParent>>,
    q_crafter: Query<(&Crafter, &Inventory)>,
) {
    let parent = q_window_parent.single();

    let entity = listener.listener();
    let (crafter, inventory) = q_crafter.get(entity).unwrap();

    commands
        .entity(parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(HudWindow::default()).with_children(|c| {
                // List recipes
                for recipe in crafter.possible_recipes() {
                    let callback = {
                        let recipe = recipe.clone();
                        move |_event: &mut ListenerInput<Pointer<Click>>, crafter: &mut Crafter| {
                            crafter.set_recipe(recipe.clone());
                        }
                    };

                    c.spawn(UiButtonBundle::new(
                        On::<Pointer<Click>>::target_component_mut::<Crafter>(callback),
                    ))
                    .with_children(|c| {
                        c.spawn(TextBundle::from_section(
                            RECIPES[recipe].text(),
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
