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
    Ship, SolarSystem,
};

pub const SAVE_DIR: &str = "assets/saves";
pub const SAVE_DIR_ASSETS_RELATIVE: &str = "saves";
pub const SAVE_FILE_EXTENSION: &str = "astras.ron";

pub fn save_solar_system(
    root: Query<Entity, With<SolarSystem>>,
    children: Query<&Children, Without<Ship>>,
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
            .extract_entity(root.single())
            .extract_entities(
                children.iter_descendants(root.single()), // .filter(|entity| world.get::<MainCamera>(*entity).is_none()),
            )
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

pub fn load_scene_system(
    mut commands: Commands,
    load_game: Option<Res<LoadGame>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(load_game) = load_game {
        if load_game.is_added() {
            commands.remove_resource::<LoadGame>();

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
