use std::{fs::File, io::Write, time::UNIX_EPOCH};

use bevy::{
    prelude::*,
    render::camera::{CameraMainTextureUsages, CameraRenderGraph},
    scene::SceneInstance,
    tasks::IoTaskPool,
};

use crate::{
    ui::Hud,
    universe::{
        AsteroidMaterial, BackgroundMaterial, LaserMaterial, PlanetMaterial, Ship, SolarSystem,
        StarMaterial,
    },
    GameState,
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
    save_game: Res<SaveGame>,
    solar_system: Single<Entity, With<SolarSystem>>,
    q_children: Query<&Children, Without<Ship>>,
    world: &World,
) {
    if save_game.is_added() {
        commands.remove_resource::<SaveGame>();
        println!("Saving scene: {}", save_game.0);

        let type_registry_arc = &**world.resource::<AppTypeRegistry>();
        let type_registry = type_registry_arc.read();

        let scene = DynamicSceneBuilder::from_world(world)
            .deny_all_resources()
            .allow_all_components()
            .allow_resource::<SaveName>()
            .deny_component::<CameraRenderGraph>()
            .deny_component::<CameraMainTextureUsages>()
            .deny_component::<MeshMaterial2d<PlanetMaterial>>()
            .deny_component::<MeshMaterial2d<StarMaterial>>()
            .deny_component::<MeshMaterial2d<AsteroidMaterial>>()
            .deny_component::<MeshMaterial2d<LaserMaterial>>()
            .deny_component::<MeshMaterial2d<BackgroundMaterial>>()
            .deny_component::<Mesh2d>()
            .deny_component::<Sprite>()
            .extract_resources()
            .extract_entity(*solar_system)
            .extract_entities(q_children.iter_descendants(*solar_system))
            .remove_empty_entities()
            .build();

        match scene.serialize(&type_registry) {
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
                eprintln!("Error while serializing the scene: {e:?}");
            }
        }
    }
}

pub fn load_solar_system(
    mut commands: Commands,
    load_game: Res<LoadGame>,
    asset_server: Res<AssetServer>,
    hud: Single<Entity, With<Hud>>,
    solar_system: Single<Entity, With<SolarSystem>>,
) {
    if load_game.is_added() {
        commands.remove_resource::<LoadGame>();
        println!("Loading scene: {}", load_game.0);

        // Remove the current SolarSystem
        commands.entity(*solar_system).despawn_recursive();

        // HUD will be recreated when Ship is Added<>
        commands.entity(*hud).despawn_recursive();

        commands.spawn((
            DynamicSceneForLoading,
            DynamicSceneRoot(asset_server.load(format!(
                "{SAVE_DIR_ASSETS_RELATIVE}/{}.{SAVE_FILE_EXTENSION}",
                load_game.0.clone()
            ))),
        ));
    }
}

// Called when DynamicSceneForLoading is fully loaded (= Added<SceneInstance>)
pub fn finish_load_solar_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    solar_system: Single<Entity, (Added<SolarSystem>, With<Parent>)>,
    dynamic_scene: Single<Entity, (With<DynamicSceneForLoading>, Added<SceneInstance>)>,
) {
    commands.entity(*solar_system).remove_parent();
    commands.entity(*dynamic_scene).despawn_recursive();

    next_state.set(GameState::GameSolarSystem);
}
