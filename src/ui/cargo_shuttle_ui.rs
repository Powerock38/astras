use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::LogisticFreight,
    items::Inventory,
    ui::{spawn_inventory_ui, UIWindow, UIWindowParent},
};

pub fn spawn_cargo_shuttle_ui(
    mut commands: Commands,
    listener: Listener<Pointer<Click>>,
    q_ui_window_parent: Query<Entity, With<UIWindowParent>>,
    q_cargo_shuttle: Query<(&LogisticFreight, &Inventory)>,
) {
    let parent = q_ui_window_parent.single();
    let (freight, inventory) = q_cargo_shuttle.get(listener.listener()).unwrap();

    commands
        .entity(parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(UIWindow::default()).with_children(|c| {
                // Inventory
                spawn_inventory_ui(c, inventory);
            });
        });
}
