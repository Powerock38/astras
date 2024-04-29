use bevy::prelude::*;

use crate::{
    ui::{spawn_inventory_ui, HudWindow, HudWindowParent},
    universe::Ship,
};

pub fn spawn_ship_ui(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_window_parent: Query<Entity, With<HudWindowParent>>,
    q_ship: Query<Entity, With<Ship>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        let Some(entity) = q_ship.iter().next() else {
            return;
        };

        let parent = q_window_parent.single();

        commands
            .entity(parent)
            .despawn_descendants()
            .with_children(|c| {
                c.spawn(HudWindow::default()).with_children(|c| {
                    spawn_inventory_ui(c, entity);
                });
            });
    }
}
