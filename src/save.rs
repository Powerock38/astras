use bevy::{
    prelude::*,
    render::camera::{CameraMainTextureUsages, CameraRenderGraph},
    scene::SceneInstance,
    sprite::Mesh2dHandle,
    tasks::IoTaskPool,
};
use std::{fs::File, io::Write, time::UNIX_EPOCH};

use crate::{
    ui::Hud,
    universe::{BackgroundMaterial, PlanetMaterial, Ship, SolarSystem, StarMaterial},
};

pub const SAVE_DIR: &str = "assets/saves";
pub const SAVE_DIR_ASSETS_RELATIVE: &str = "saves";
pub const SAVE_FILE_EXTENSION: &str = "astras.ron";

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct SaveName(pub String);

#[derive(Resource)]
pub struct SaveGame(pub String);

#[derive(Resource)]
pub struct LoadGame(pub String);

#[derive(Component)]
pub struct DynamicSceneForLoading;

pub fn save_key_shortcut(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    save_name: Res<SaveName>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyL) {
        let timestamp = UNIX_EPOCH.elapsed().unwrap().as_millis();
        commands.insert_resource(SaveGame(format!("{}-{timestamp}", save_name.0.clone())));
    }
}

pub fn save_solar_system(
    mut commands: Commands,
    save_game: Option<Res<SaveGame>>,
    q_solar_system: Query<Entity, With<SolarSystem>>,
    q_children: Query<&Children, Without<Ship>>,
    world: &World,
) {
    if let Some(save_game) = save_game {
        if save_game.is_added() {
            commands.remove_resource::<SaveGame>();
            println!("Saving scene: {}", save_game.0);

            let app_type_registry = world.resource::<AppTypeRegistry>().clone();

            let solar_system = q_solar_system.single();

            let scene = DynamicSceneBuilder::from_world(world)
                .deny_all_resources()
                .allow_resource::<SaveName>()
                .allow_all()
                .deny::<CameraRenderGraph>()
                .deny::<CameraMainTextureUsages>()
                .deny::<Handle<PlanetMaterial>>()
                .deny::<Handle<StarMaterial>>()
                .deny::<Handle<BackgroundMaterial>>()
                .deny::<Handle<Image>>()
                .deny::<Mesh2dHandle>()
                .deny::<Sprite>()
                .extract_entity(solar_system)
                .extract_entities(q_children.iter_descendants(solar_system))
                .remove_empty_entities()
                .build();

            match scene.serialize_ron(&app_type_registry) {
                Ok(serialized) => {
                    let save_name = save_game.0.clone();

                    IoTaskPool::get()
                        .spawn(async move {
                            File::create(format!("{SAVE_DIR}/{save_name}.{SAVE_FILE_EXTENSION}"))
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
}

pub fn load_solar_system(
    mut commands: Commands,
    load_game: Option<Res<LoadGame>>,
    asset_server: Res<AssetServer>,
    q_hud: Query<Entity, With<Hud>>,
    q_solar_system: Query<Entity, With<SolarSystem>>,
) {
    if let Some(load_game) = load_game {
        if load_game.is_added() {
            commands.remove_resource::<LoadGame>();
            println!("Loading scene: {}", load_game.0);

            // Remove the current SolarSystem
            commands.entity(q_solar_system.single()).despawn_recursive();

            // HUD will be recreated when Ship is Added<>
            commands.entity(q_hud.single()).despawn_recursive();

            commands.spawn((
                DynamicSceneForLoading,
                DynamicSceneBundle {
                    scene: asset_server.load(format!(
                        "{SAVE_DIR_ASSETS_RELATIVE}/{}.{SAVE_FILE_EXTENSION}",
                        load_game.0.clone()
                    )),
                    ..default()
                },
            ));
        }
    }
}

// Called when DynamicSceneForLoading is fully loaded (= Added<SceneInstance>)
pub fn finish_load_solar_system(
    mut commands: Commands,
    q_solar_system: Query<Entity, (Added<SolarSystem>, With<Parent>)>,
    q_dynamic_scene: Query<Entity, (With<DynamicSceneForLoading>, Added<SceneInstance>)>,
) {
    let Some(solar_system) = q_solar_system.iter().next() else {
        return;
    };

    commands.entity(solar_system).remove_parent();

    if let Some(dynamic_scene) = q_dynamic_scene.iter().next() {
        commands.entity(dynamic_scene).despawn_recursive();
    }
}
