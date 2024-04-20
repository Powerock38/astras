use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::LogisticFreight,
    items::Inventory,
    ui::{spawn_inventory_ui, HudWindow, HudWindowParent},
};

pub fn spawn_cargo_shuttle_ui(
    mut commands: Commands,
    listener: Listener<Pointer<Click>>,
    q_window_parent: Query<Entity, With<HudWindowParent>>,
    q_cargo_shuttle: Query<(&LogisticFreight, &Inventory)>,
) {
    let parent = q_window_parent.single();
    let (_freight, inventory) = q_cargo_shuttle.get(listener.listener()).unwrap();

    commands
        .entity(parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(HudWindow::default()).with_children(|c| {
                // Inventory
                spawn_inventory_ui(c, inventory);
            });
        });
}
