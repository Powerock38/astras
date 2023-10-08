use crate::constants::COLORS;
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle},
};
use rand::{seq::SliceRandom, Rng};

#[derive(Component, Debug)]
pub struct Astre {
    pub name: String,
    pub mass: f32,
    pub radius: f32,
    pub orbit_speed: f32,
    pub orbit_direction: bool,
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(0)]
    pub seed: f32,
    #[uniform(0)]
    pub scale: f32,
    #[uniform(0)]
    pub u: f32,
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
}

pub fn spawn_astre(
    c: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<PlanetMaterial>>,
    angle: f32,
    radius: f32,
    mass: f32,
    position: Vec2,
    orbit_speed: f32,
    orbit_direction: bool,
    nb_children: u32,
    z_value: u32,
) {
    let gravity_range = mass + radius;

    let mut rng = rand::thread_rng();

    let color = COLORS.choose(&mut rng).unwrap();

    let u = 1. - (rng.gen::<f32>() - 1.).powf(4.);

    let material = PlanetMaterial {
        color: color.clone(),
        seed: rng.gen::<f32>() * 1000.,
        scale: rng.gen_range(1.0..10.0),
        u,
    };

    //let nb_sides = rand::thread_rng().gen_range(4..=12);
    //let mesh = shape::RegularPolygon::new(radius, nb_sides);
    let mesh = shape::Circle {
        radius,
        ..Default::default()
    };

    let transform = Transform::from_translation(position.extend(z_value as f32));

    let astre = Astre {
        name: String::from("Astre"),
        mass,
        radius,
        orbit_speed,
        orbit_direction,
    };

    let mut orbit_distance = gravity_range;

    c.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh.into()).into(),
        material: materials.add(material),
        transform,
        ..default()
    })
    .insert(astre)
    .with_children(|c| {
        for i in 0..nb_children {
            let child_nb_children =
                rand::thread_rng().gen_range(0..=(0.1 * nb_children as f32) as u32);

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
                z_value + 1,
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
