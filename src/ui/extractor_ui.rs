use bevy::prelude::*;

use crate::{
    buildings::Extractor,
    ui::{HudWindow, HudWindowParent, InventoryUI, build_building_header},
};

pub fn scan_extractor_ui(mut commands: Commands, q_extractors: Query<Entity, Added<Extractor>>) {
    for entity in &q_extractors {
        commands.entity(entity).observe(spawn_extractor_ui);
    }
}

fn spawn_extractor_ui(
    pointer_click: On<Pointer<Click>>,
    mut commands: Commands,
    window_parent: Single<Entity, With<HudWindowParent>>,
) {
    commands
        .entity(*window_parent)
        .despawn_related::<Children>()
        .with_children(|c| {
            c.spawn((
                HudWindow,
                children![
                    build_building_header("Element Extractor"),
                    InventoryUI::new(pointer_click.entity)
                ],
            ));
        });
}
