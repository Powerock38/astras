use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::Material2d,
};
use rand::prelude::*;

use crate::{
    handle_loader::{MaterialLoader, MeshType},
    items::{ElementOnAstre, ElementState, Inventory},
    universe::{Astre, Orbit},
};

const ASTEROID_MIN_RADIUS: f32 = 50.0;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Astre, Orbit, MaterialLoader<AsteroidMaterial>)]
pub struct Asteroid {
    initial_size: u32,
    rotation_speed: f32,
}

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect, Default)]
pub struct AsteroidMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub seed: f32,
}

impl Material2d for AsteroidMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/asteroid.wgsl".into()
    }
}

pub fn build_asteroid_belt(c: &mut ChildBuilder, rng: &mut StdRng) {
    let radius: f32 = rng.gen_range(10_000.0..100_000.0);
    let nb_asteroids = rng.gen_range(10..100);

    let radius_variation = rng.gen_range(100.0..radius * 0.2);

    for i in 0..nb_asteroids {
        let angle = (i as f32 / nb_asteroids as f32) * 2. * PI;

        let local_radius = rng.gen_range(radius - radius_variation..radius + radius_variation);

        let z = i as f32 / nb_asteroids as f32;
        let position = Vec3::new(local_radius * angle.cos(), local_radius * angle.sin(), z);

        build_asteroid(c, rng, position);
    }
}

fn build_asteroid(c: &mut ChildBuilder, rng: &mut StdRng, position: Vec3) {
    let seed_asteroid = rng.gen::<u64>();
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed_asteroid);

    let avg_radius = rng.gen_range(2.0 * ASTEROID_MIN_RADIUS..1000.0);

    let transform = Transform::from_translation(position);

    // Asteroids have only one element
    let composition = ElementOnAstre::random_elements(
        &mut rng,
        1,
        avg_radius as u32 * 10,
        &[ElementState::Solid],
    );

    let initial_size = composition[0].quantity;

    let color = ElementOnAstre::get_color(&composition);

    let rotation_speed = rng.gen_range(-0.2..0.2);

    let material = AsteroidMaterial {
        color,
        seed: rng.gen::<f32>() * 1000.,
    };

    c.spawn((
        Name::new("Asteroid"),
        Asteroid {
            initial_size,
            rotation_speed,
        },
        Astre::new(avg_radius, 0.0),
        Inventory::from(composition),
        Orbit::new(&mut rng),
        MaterialLoader {
            material,
            mesh_type: MeshType::RandomPolygon(seed_asteroid, avg_radius),
        },
        transform,
    ));
}

pub fn random_polygon(seed: u64, avg_radius: f32) -> Mesh {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let num_vertices = rng.gen_range(7..20);
    let irregularity = rng.gen_range(0.0..1.0);
    let spikiness = rng.gen_range(0.0..0.7);

    let irregularity = irregularity * 2.0 * PI / (num_vertices as f32);
    let spikiness = spikiness * avg_radius;

    // generate n angle steps
    let mut angle_steps = Vec::new();
    let lower = 2.0 * PI / (num_vertices as f32) - irregularity;
    let upper = 2.0 * PI / (num_vertices as f32) + irregularity;
    let mut cumsum = 0.0;
    for _ in 0..num_vertices {
        let angle = rng.gen_range(lower..upper);
        angle_steps.push(angle);
        cumsum += angle;
    }

    // normalize the steps so that point 0 and point n+1 are the same
    cumsum /= 2.0 * PI;
    for angle in &mut angle_steps {
        *angle /= cumsum;
    }

    // now generate the points
    let mut points = Vec::new();
    let mut angle = rng.gen_range(0.0..2.0 * PI);
    for _ in 0..num_vertices {
        let radius = rng
            .gen_range((avg_radius - spikiness)..(avg_radius + spikiness))
            .max(0.0)
            .min(2.0 * avg_radius);
        let point = Vec2::new(radius * angle.cos(), radius * angle.sin());
        points.push(point);
        angle += angle_steps[points.len() - 1];
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let mut indices = Vec::new();

    // Triangulate the polygon
    for i in 1..num_vertices - 1 {
        indices.push(0);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        points
            .iter()
            .map(|p| [p.x, p.y, 0.0])
            .collect::<Vec<[f32; 3]>>(),
    );

    mesh.insert_indices(Indices::U32(indices));

    // Generate UV coordinates based on vertex positions
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        points
            .iter()
            .map(|p| {
                let u = (p.x / (2.0 * avg_radius)) + 0.5;
                let v = (p.y / (2.0 * avg_radius)) + 0.5;
                [u, v]
            })
            .collect::<Vec<[f32; 2]>>(),
    );

    mesh
}

pub fn update_asteroids(
    mut commands: Commands,
    time: Res<Time>,
    mut q_asteroids: Query<(Entity, &Asteroid, &Astre, &mut Transform, &Inventory)>,
) {
    for (entity, asteroid, astre, mut transform, inventory) in &mut q_asteroids {
        // Rotate the asteroid
        transform.rotate(Quat::from_rotation_z(
            asteroid.rotation_speed * time.delta_secs(),
        ));

        // Mining the asteroid shrinks it, until it disappears
        let scale = inventory.total_quantity() as f32 / asteroid.initial_size as f32;

        if scale * astre.surface_radius() < ASTEROID_MIN_RADIUS {
            commands.entity(entity).despawn_recursive();
        } else {
            transform.scale = Vec3::splat(scale);
        }
    }
}
