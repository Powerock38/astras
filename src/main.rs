use astre::*;
use bevy::prelude::*;
use rand::Rng;
use ship::*;

mod astre;
mod ship;
mod constants;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_universe)
        .add_system(update_astres)
        .add_system(update_ship)
        .add_system(update_camera)
        .run();
}

fn setup_universe(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_solar_system(commands, meshes, materials)
}

#[derive(Component)]
pub struct SolarSystem;

fn spawn_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = rand::thread_rng().gen_range((300.)..1000.);
    let mass = rand::thread_rng().gen_range((100.)..3000.);
    let position = Vec2::new(0., 0.);
    let nb_children = rand::thread_rng().gen_range(3..=10);

    commands
        .spawn(SpatialBundle::default())
        .insert(SolarSystem)
        .with_children(|mut c| {
            spawn_astre(
                &mut c,
                &mut meshes,
                &mut materials,
                0.,
                radius,
                mass,
                position,
                0.,
                true,
                nb_children,
            );
        })
        .with_children(|c| {
            setup_ship(c, meshes, materials);
        });
}
