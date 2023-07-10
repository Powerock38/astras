use crate::constants::COLORS;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::{seq::SliceRandom, Rng};

#[derive(Component)]
pub struct Astre {
    pub name: String,
    pub mass: f32,
    pub radius: f32,
    pub orbit_speed: f32,
    pub orbit_direction: bool,
}

pub fn spawn_astre(
    c: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    angle: f32,
    radius: f32,
    mass: f32,
    position: Vec2,
    orbit_speed: f32,
    orbit_direction: bool,
    nb_children: u32,
) {
    let mut rng = rand::thread_rng();

    let color = COLORS.choose(&mut rng).unwrap();
    let material = ColorMaterial::from(color.clone());

    let mesh = shape::RegularPolygon::new(radius, 6);

    let transform = Transform::from_translation(position.extend(0.));

    let astre = Astre {
        name: String::from("Astre"),
        mass,
        radius,
        orbit_speed,
        orbit_direction,
    };

    let mut orbit_distance = radius + mass;

    c.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh.into()).into(),
        material: materials.add(material),
        transform,
        ..default()
    })
    .insert(astre)
    .with_children(|c| {
        for i in 0..nb_children {

            let max_nb_children = (nb_children - 1).max(0);

            let child_nb_children = rand::thread_rng().gen_range(0..=max_nb_children);

            let child_angle = (i as f32 / nb_children as f32) * 2. * std::f32::consts::PI;

            let child_mass = rand::thread_rng().gen_range((mass * 0.1)..(mass * 0.9));

            let child_radius = rand::thread_rng().gen_range((radius * 0.1)..(radius * 0.7));

            let child_orbit_speed = rand::thread_rng()
                .gen_range((std::f32::consts::PI / 100.)..=std::f32::consts::PI / 10.);

            let child_orbit_direction = rand::thread_rng().gen_bool(0.5);

            orbit_distance += child_radius + child_mass;

            let position = Vec2::new(orbit_distance * angle.cos(), orbit_distance * angle.sin());

            spawn_astre(
                c,
                meshes,
                materials,
                child_angle,
                child_radius,
                child_mass,
                position,
                child_orbit_speed,
                child_orbit_direction,
                child_nb_children,
            );

            orbit_distance += rand::thread_rng().gen_range((child_mass * 0.2)..=(child_mass * 1.5));
        }
    });
}

pub fn update_astres(time: Res<Time>, mut astre_query: Query<(&Astre, &mut Transform)>) {
    for (astre, mut transform) in astre_query.iter_mut() {
        let angle = transform.translation.y.atan2(transform.translation.x);
        let orbit = (transform.translation.x.powf(2.0) + transform.translation.y.powf(2.0)).sqrt();

        let direction = if astre.orbit_direction { 1. } else { -1. };

        let orbit_angle = angle + direction * astre.orbit_speed * time.delta_seconds();

        transform.translation =
            Vec2::new(orbit * orbit_angle.cos(), orbit * orbit_angle.sin()).extend(0.);
    }
}
