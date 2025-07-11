use bevy::prelude::*;

use crate::{
    buildings::Spaceport,
    ui::{build_building_header, HudWindow, HudWindowParent, InventoryUI},
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
    let entity = trigger.target();

    commands
        .entity(*window_parent)
        .despawn_related::<Children>()
        .with_children(|c| {
            c.spawn((
                HudWindow,
                children![
                    build_building_header("Spaceport"),
                    InventoryUI::new(entity).with_edit_logistic()
                ],
            ));
        });
}
