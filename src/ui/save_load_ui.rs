use bevy::prelude::*;

use crate::{
    ui::{HudWindow, HudWindowParent, UiButton},
    LoadGame, SAVE_DIR, SAVE_FILE_EXTENSION,
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
    if let Ok(save_files) = std::fs::read_dir(SAVE_DIR).map(|dir| {
        dir.filter_map(|entry| {
            entry.ok().and_then(|entry| {
                entry.file_name().into_string().ok().and_then(|file_name| {
                    if file_name.ends_with(SAVE_FILE_EXTENSION) {
                        Some(file_name.replacen(&format!(".{SAVE_FILE_EXTENSION}"), "", 1))
                    } else {
                        None
                    }
                })
            })
        })
        .collect::<Vec<_>>()
    }) {
        for save_file in save_files {
            let callback = {
                let save_file = save_file.clone();
                move |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.insert_resource(LoadGame(save_file.clone()));
                }
            };

            c.spawn(UiButton).observe(callback).with_children(|c| {
                c.spawn(Text::new(save_file));
            });
        }
    }
}
