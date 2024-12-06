use bevy::prelude::*;

use crate::{
    buildings::Extractor,
    ui::{spawn_building_header, spawn_inventory_ui, HudWindow, HudWindowParent},
};

pub fn scan_extractor_ui(mut commands: Commands, q_extractors: Query<Entity, Added<Extractor>>) {
    for entity in &q_extractors {
        commands.entity(entity).observe(spawn_extractor_ui);
    }
}

fn spawn_extractor_ui(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    window_parent: Single<Entity, With<HudWindowParent>>,
) {
    let entity = trigger.entity();

    commands
        .entity(*window_parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(HudWindow).with_children(|c| {
                spawn_building_header(c, entity, "Element Extractor");

                spawn_inventory_ui(c, entity);
            });
        });
}
