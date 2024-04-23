use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::Spaceport,
    ui::{spawn_inventory_ui, HudWindow, HudWindowParent},
};

pub fn scan_spaceport_ui(mut commands: Commands, q_extractor: Query<Entity, Added<Spaceport>>) {
    for entity in q_extractor.iter() {
        commands
            .entity(entity)
            .insert(On::<Pointer<Click>>::run(spawn_spaceport_ui));
    }
}

pub fn spawn_spaceport_ui(
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
                //TODO: UI to build LogisticRequest / LogisticProvider of each scope (Planet, SolarSystem)

                spawn_inventory_ui(c, entity);
            });
        });
}
