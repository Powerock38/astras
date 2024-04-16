use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::{
    ui::{HudWindow, HudWindowParent, UiButtonBundle},
    LoadGame, SAVE_DIR, SAVE_FILE_EXTENSION,
};

pub fn spawn_save_ui(
    mut commands: Commands,
    q_window_parent: Query<Entity, With<HudWindowParent>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        let parent = q_window_parent.single();

        commands
            .entity(parent)
            .despawn_descendants()
            .with_children(|c| {
                c.spawn(HudWindow::default()).with_children(|c| {
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
                move |_event: &mut ListenerInput<Pointer<Click>>,
                      target_commands: &mut EntityCommands| {
                    target_commands
                        .commands()
                        .insert_resource(LoadGame(save_file.clone()));
                }
            };

            c.spawn(UiButtonBundle::new(
                On::<Pointer<Click>>::target_commands_mut(callback),
            ))
            .with_children(|c| {
                c.spawn(TextBundle::from_section(
                    save_file,
                    TextStyle {
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ));
            });
        }
    }
}
