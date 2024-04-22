use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::Extractor,
    ui::{spawn_inventory_ui, HudWindow, HudWindowParent},
};

pub fn scan_extractor_ui(mut commands: Commands, q_extractor: Query<Entity, Added<Extractor>>) {
    for entity in q_extractor.iter() {
        commands
            .entity(entity)
            .insert(On::<Pointer<Click>>::run(spawn_extractor_ui));
    }
}

pub fn spawn_extractor_ui(
    mut commands: Commands,
    listener: Listener<Pointer<Click>>,
    q_window_parent: Query<Entity, With<HudWindowParent>>,
) {
    let parent = q_window_parent.single();
    let entity = listener.listener();

    commands
        .entity(parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(HudWindow::default()).with_children(|c| {
                spawn_inventory_ui(c, entity);
            });
        });
}
