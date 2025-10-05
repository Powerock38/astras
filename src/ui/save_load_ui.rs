use bevy::prelude::*;

use crate::{
    ui::{HudWindow, HudWindowParent, UiButton},
    LoadUniverse, SAVES_DIR, SAVE_EXTENSION,
};

pub fn spawn_save_ui(
    mut commands: Commands,
    window_parent: Single<Entity, With<HudWindowParent>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        commands
            .entity(*window_parent)
            .despawn_related::<Children>()
            .with_children(|c| {
                c.spawn(HudWindow).with_children(|c| {
                    build_load_ui(c);
                });
            });
    }
}

pub fn build_load_ui(c: &mut ChildSpawnerCommands) {
    if let Ok(universes_names) = std::fs::read_dir(format!("assets/{SAVES_DIR}")).map(|dir| {
        dir.filter_map(|entry| {
            entry.ok().and_then(|entry| {
                entry.file_name().into_string().ok().and_then(|file_name| {
                    if file_name.ends_with(SAVE_EXTENSION) {
                        Some(file_name.replacen(&format!(".{SAVE_EXTENSION}"), "", 1))
                    } else {
                        None
                    }
                })
            })
        })
        .collect::<Vec<_>>()
    }) {
        for universe_name in universes_names {
            let callback = {
                let universe_name = universe_name.clone();
                move |_pointer_click: On<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(LoadUniverse(universe_name.clone()));
                }
            };

            c.spawn((UiButton, children![Text::new(universe_name)]))
                .observe(callback);
        }
    }
}
