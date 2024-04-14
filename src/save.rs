use bevy::{
    prelude::*,
    render::camera::{CameraMainTextureUsages, CameraRenderGraph},
    sprite::Mesh2dHandle,
    tasks::IoTaskPool,
};
use std::{fs::File, io::Write, time::UNIX_EPOCH};

use crate::{
    astres::{PlanetMaterial, StarMaterial},
    background::BackgroundMaterial,
    ui::Hud,
    Ship, SolarSystem,
};

pub const SAVE_DIR: &str = "assets/saves";
pub const SAVE_DIR_ASSETS_RELATIVE: &str = "saves";
pub const SAVE_FILE_EXTENSION: &str = "astras.ron";

pub fn save_solar_system(
    q_root: Query<Entity, With<SolarSystem>>,
    q_children: Query<&Children, Without<Ship>>,
    world: &World,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyL) {
        let app_type_registry = world.resource::<AppTypeRegistry>().clone();

        let scene = DynamicSceneBuilder::from_world(world)
            .deny_all_resources()
            .allow_all()
            .deny::<CameraRenderGraph>()
            .deny::<CameraMainTextureUsages>()
            .deny::<Handle<PlanetMaterial>>()
            .deny::<Handle<StarMaterial>>()
            .deny::<Handle<BackgroundMaterial>>()
            .deny::<Handle<Image>>()
            .deny::<Mesh2dHandle>()
            .deny::<Sprite>()
            .extract_entity(q_root.single())
            .extract_entities(q_children.iter_descendants(q_root.single()))
            .remove_empty_entities()
            .build();

        match scene.serialize_ron(&app_type_registry) {
            Ok(serialized) => {
                IoTaskPool::get()
                    .spawn(async move {
                        let timestamp = UNIX_EPOCH.elapsed().unwrap().as_millis();

                        File::create(format!("{SAVE_DIR}/{timestamp}.{SAVE_FILE_EXTENSION}"))
                            .and_then(|mut file| file.write(serialized.as_bytes()))
                            .expect("Error while writing scene to file");
                    })
                    .detach();
            }
            Err(e) => {
                eprintln!("Error while serializing the scene: {:?}", e);
            }
        }
    }
}

#[derive(Resource)]
pub struct LoadGame(pub String);

//FIXME: loading a non-new save is bugged (black screen)
pub fn load_scene_system(
    mut commands: Commands,
    load_game: Option<Res<LoadGame>>,
    asset_server: Res<AssetServer>,
    q_root: Query<Entity, With<SolarSystem>>,
    q_hud: Query<Entity, With<Hud>>,
) {
    if let Some(load_game) = load_game {
        if load_game.is_added() {
            commands.remove_resource::<LoadGame>();

            commands.entity(q_hud.single()).despawn_recursive();

            commands.entity(q_root.single()).despawn_recursive();

            println!("Loading scene: {}", load_game.0);

            commands.spawn(DynamicSceneBundle {
                scene: asset_server.load(format!(
                    "{SAVE_DIR_ASSETS_RELATIVE}/{}",
                    load_game.0.clone()
                )),
                ..default()
            });
        }
    }
}
