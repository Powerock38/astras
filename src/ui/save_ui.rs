use bevy::prelude::*;

use crate::{
    ui::{HudButtonAction, HudButtonBundle, UIWindow, UIWindowParent},
    SAVE_DIR, SAVE_FILE_EXTENSION,
};

pub fn spawn_save_ui(
    mut commands: Commands,
    q_ui_window_parent: Query<Entity, With<UIWindowParent>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        let parent = q_ui_window_parent.single();

        commands
            .entity(parent)
            .despawn_descendants()
            .with_children(|c| {
                c.spawn(UIWindow::default()).with_children(|c| {
                    if let Ok(save_files) = std::fs::read_dir(SAVE_DIR).map(|dir| {
                        dir.filter_map(|entry| {
                            entry.ok().and_then(|entry| {
                                entry.file_name().into_string().ok().and_then(|file_name| {
                                    if file_name.ends_with(SAVE_FILE_EXTENSION) {
                                        Some(file_name)
                                    } else {
                                        None
                                    }
                                })
                            })
                        })
                        .collect::<Vec<_>>()
                    }) {
                        for save_file in save_files {
                            c.spawn(HudButtonBundle::new(HudButtonAction::LoadGame(
                                save_file.clone(),
                            )))
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
                });
            });
    }
}
