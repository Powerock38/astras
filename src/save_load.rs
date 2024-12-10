use std::{fs::File, io::Write, path::Path};

use bevy::{
    prelude::*,
    render::camera::{CameraMainTextureUsages, CameraRenderGraph},
    tasks::IoTaskPool,
};

use crate::{
    ui::Hud,
    universe::{
        AsteroidMaterial, BackgroundMaterial, DockableOnAstre, LaserMaterial, PlanetMaterial, Ship,
        SolarSystem, StarMaterial,
    },
    GameState,
};

pub const SAVES_DIR: &str = "saves";
const SHIP_SAVE_FILENAME: &str = "ship";
pub const SAVE_EXTENSION: &str = "scn.ron";

#[derive(Resource)]
pub struct UniverseName(pub String);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct CurrentSolarSystemName(pub String);

#[derive(Event)]
pub struct SaveShip;

#[derive(Event)]
pub struct SaveSolarSystem;

#[derive(Event)]
pub struct LoadUniverse(pub String);

fn write_scene_file(serialized: Result<String, bevy::scene::ron::Error>, path: String) {
    match serialized {
        Ok(serialized) => {
            IoTaskPool::get()
                .spawn(async move {
                    let path = Path::new(&path);
                    path.parent()
                        .and_then(|p| std::fs::create_dir_all(p).ok())
                        .expect("Error while creating directory for save file");

                    File::create(path)
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

pub fn save_ship(
    _trigger: Trigger<SaveShip>,
    mut commands: Commands,
    q_ship: Single<
        (
            Entity,
            &mut Transform,
            &GlobalTransform,
            &mut DockableOnAstre,
        ),
        With<Ship>,
    >,
) {
    let (ship_entity, mut transform, global_transform, mut dockable_on_astre) = q_ship.into_inner();

    // Remove the ship from its parent to save it at the root of the scene

    *transform = global_transform.reparented_to(&GlobalTransform::default());
    dockable_on_astre.on_astre = false;
    commands
        .entity(ship_entity)
        .observe(save_ship_inner)
        .remove_parent();
}

pub fn save_ship_inner(
    trigger: Trigger<OnRemove, Parent>,
    mut commands: Commands,
    universe_name: Res<UniverseName>,
    world: &World,
) {
    commands.entity(trigger.observer()).despawn();

    let ship_entity = trigger.entity();
    let universe_name = universe_name.0.clone();

    println!("Saving ship in universe {universe_name}");

    let type_registry_arc = &**world.resource::<AppTypeRegistry>();
    let type_registry = type_registry_arc.read();

    let scene = DynamicSceneBuilder::from_world(world)
        .deny_all_resources()
        .allow_resource::<CurrentSolarSystemName>()
        .allow_all_components()
        .deny_component::<Mesh2d>()
        .deny_component::<Sprite>()
        .deny_component::<Children>()
        .deny_component::<Parent>()
        .extract_entity(ship_entity)
        .extract_resources()
        .build();

    write_scene_file(
        scene.serialize(&type_registry),
        format!("assets/{SAVES_DIR}/{universe_name}/{SHIP_SAVE_FILENAME}.{SAVE_EXTENSION}"),
    );
}

pub fn save_solar_system(
    _trigger: Trigger<SaveSolarSystem>,
    universe_name: Res<UniverseName>,
    q_solar_system: Single<(Entity, &SolarSystem)>,
    ship_entity: Single<Entity, With<Ship>>, // Filter out the Ship entity
    q_children: Query<&Children, Without<Ship>>, // Filter out the Ship children (sprite, camera)
    world: &World,
) {
    let universe_name = universe_name.0.clone();

    let (solar_system_entity, solar_system) = q_solar_system.into_inner();
    let solar_system_name = solar_system.name();

    println!("Saving solar system {solar_system_name} in universe {universe_name}");

    let type_registry_arc = &**world.resource::<AppTypeRegistry>();
    let type_registry = type_registry_arc.read();

    let scene = DynamicSceneBuilder::from_world(world)
        .deny_all_resources()
        .allow_all_components()
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
        .extract_entity(solar_system_entity)
        .extract_entities(
            q_children
                .iter_descendants(solar_system_entity)
                .filter(|e| *e != *ship_entity),
        )
        .remove_empty_entities()
        .build();

    write_scene_file(
        scene.serialize(&type_registry),
        format!("assets/{SAVES_DIR}/{universe_name}/{solar_system_name}.{SAVE_EXTENSION}"),
    );
}

// Load universe = load ship => which inserts CurrentSolarSystemName => load solar system
pub fn load_universe(
    trigger: Trigger<LoadUniverse>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    hud: Option<Single<Entity, With<Hud>>>,
    solar_system: Option<Single<Entity, With<SolarSystem>>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::LoadingSave);

    // Remove the current SolarSystem
    if let Some(solar_system) = solar_system {
        commands.entity(*solar_system).despawn_recursive();
    }

    // HUD will be recreated when Ship is Added<>
    if let Some(hud) = hud {
        commands.entity(*hud).despawn_recursive();
    }

    commands.remove_resource::<CurrentSolarSystemName>();

    let universe_name = trigger.0.clone();
    println!("Loading universe {universe_name}");

    commands.insert_resource(UniverseName(universe_name.clone()));

    scene_spawner.spawn_dynamic(asset_server.load(format!(
        "{SAVES_DIR}/{universe_name}/{SHIP_SAVE_FILENAME}.{SAVE_EXTENSION}",
    )));

    println!("Loading ship save");
}

pub fn load_solar_system(
    asset_server: Res<AssetServer>,
    universe_name: Res<UniverseName>,
    current_solar_system_name: Res<CurrentSolarSystemName>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    if current_solar_system_name.is_added() {
        println!("Loaded ship save");

        let universe_name = universe_name.0.clone();
        let current_solar_system_name = current_solar_system_name.0.clone();

        scene_spawner.spawn_dynamic(asset_server.load(format!(
            "{SAVES_DIR}/{universe_name}/{current_solar_system_name}.{SAVE_EXTENSION}",
        )));

        println!("Loading solar system save");
    }
}

pub fn finish_load_solar_system(
    mut commands: Commands,
    solar_system: Single<Entity, Added<SolarSystem>>,
    ship_entity: Single<Entity, With<Ship>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Loaded solar system save");

    commands.entity(*ship_entity).set_parent(*solar_system);

    next_state.set(GameState::GameSolarSystem);
}
