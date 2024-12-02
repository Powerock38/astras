use bevy::prelude::*;

use crate::{
    buildings::Extractor,
    ui::{spawn_inventory_ui, HudWindow, HudWindowParent},
};

pub fn scan_extractor_ui(mut commands: Commands, q_extractor: Query<Entity, Added<Extractor>>) {
    for entity in q_extractor.iter() {
        commands.entity(entity).observe(spawn_extractor_ui);
    }
}

pub fn spawn_extractor_ui(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    q_window_parent: Query<Entity, With<HudWindowParent>>,
) {
    let parent = q_window_parent.single();
    let entity = trigger.entity();

    commands
        .entity(parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(HudWindow).with_children(|c| {
                spawn_inventory_ui(c, entity);
            });
        });
}
