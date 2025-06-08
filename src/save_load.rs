use std::{fs::File, io::Write, path::Path};

use bevy::{
    ecs::system::SystemState,
    prelude::*,
    render::camera::{CameraMainTextureUsages, CameraRenderGraph},
    tasks::IoTaskPool,
};

use crate::{
    ui::{Hud, NotificationEvent},
    universe::{
        AsteroidMaterial, BackgroundMaterial, LaserMaterial, PlanetMaterial, Ship, SolarSystem,
        StarMaterial,
    },
    GameState,
};

pub const SAVES_DIR: &str = "saves";
pub const SAVE_EXTENSION: &str = "scn.ron";

#[derive(Resource)]
pub struct UniverseName(pub String);

#[derive(Event)]
pub struct LoadUniverse(pub String);

pub struct SaveUniverse;

impl Command for SaveUniverse {
    fn apply(self, world: &mut World) {
        let mut system_state: SystemState<(
            Query<Entity, With<SolarSystem>>,
            Query<&Children, Without<Ship>>, // Filter out the Ship children (sprite, camera)
        )> = SystemState::new(world);

        let (q_solar_systems, q_children) = system_state.get_mut(world);

        let mut entities = vec![];
        for solar_system in &q_solar_systems {
            entities.push(solar_system);

            for child in q_children.iter_descendants(solar_system) {
                entities.push(child);
            }
        }

        let universe_name = world.resource::<UniverseName>().0.clone();

        info!("Saving universe {universe_name}");
        world.trigger(NotificationEvent(format!(
            "Saving universe {universe_name}"
        )));

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
            .extract_entities(entities.into_iter())
            .remove_empty_entities()
            .build();

        let path = format!("assets/{SAVES_DIR}/{universe_name}.{SAVE_EXTENSION}");

        match scene.serialize(&type_registry) {
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
                error!("Error while serializing the scene: {e:?}");
            }
        }
    }
}

pub fn load_universe(
    trigger: Trigger<LoadUniverse>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    q_solar_systems: Query<Entity, With<SolarSystem>>,
    hud: Option<Single<Entity, With<Hud>>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Remove all SolarSystems
    for solar_system in &q_solar_systems {
        commands.entity(solar_system).try_despawn();
    }

    // HUD will be recreated when Ship is Added<>
    if let Some(hud) = hud {
        commands.entity(*hud).despawn();
    }

    let universe_name = trigger.0.clone();

    commands.insert_resource(UniverseName(universe_name.clone()));

    scene_spawner
        .spawn_dynamic(asset_server.load(format!("{SAVES_DIR}/{universe_name}.{SAVE_EXTENSION}")));

    info!("Loading {universe_name}");

    next_state.set(GameState::GameSolarSystem);
}
