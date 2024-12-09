use bevy::prelude::*;

use crate::{
    ui::{HudWindow, HudWindowParent, UiButton},
    LoadUniverse, SAVES_DIR,
};

pub fn spawn_save_ui(
    mut commands: Commands,
    window_parent: Single<Entity, With<HudWindowParent>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        commands
            .entity(*window_parent)
            .despawn_descendants()
            .with_children(|c| {
                c.spawn(HudWindow).with_children(|c| {
                    build_load_ui(c);
                });
            });
    }
}

pub fn build_load_ui(c: &mut ChildBuilder) {
    if let Ok(universes_names) = std::fs::read_dir(format!("assets/{SAVES_DIR}")).map(|dir| {
        dir.filter_map(|entry| {
            entry.ok().and_then(|entry| {
                entry
                    .file_type()
                    .ok()
                    .filter(std::fs::FileType::is_dir)
                    .and_then(|_| entry.file_name().into_string().ok())
            })
        })
        .collect::<Vec<_>>()
    }) {
        for universe_name in universes_names {
            let callback = {
                let universe_name = universe_name.clone();
                move |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(LoadUniverse(universe_name.clone()));
                }
            };

            c.spawn(UiButton).observe(callback).with_children(|c| {
                c.spawn(Text::new(universe_name));
            });
        }
    }
}
