use bevy::prelude::*;

use crate::{
    buildings::Spaceport,
    ui::{spawn_inventory_ui, HudWindow, HudWindowParent},
};

pub fn scan_spaceport_ui(mut commands: Commands, q_extractors: Query<Entity, Added<Spaceport>>) {
    for entity in &q_extractors {
        commands.entity(entity).observe(spawn_spaceport_ui);
    }
}

pub fn spawn_spaceport_ui(
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
                //TODO: UI to build LogisticRequest / LogisticProvider of each scope (Planet, SolarSystem)

                spawn_inventory_ui(c, entity);
            });
        });
}
