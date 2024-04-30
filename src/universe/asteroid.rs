use crate::handle_loader::{HandleLoaderBundle, MaterialLoader, MeshType};
use crate::items::{ElementOnAstre, ElementState, ELEMENTS};
use bevy::render::mesh::Indices;
use bevy::{prelude::*, render::mesh::PrimitiveTopology, render::render_asset::RenderAssetUsages};
use rand::prelude::*;
use std::f32::consts::PI;

use crate::universe::{AstreBundle, StarMaterial};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Asteroid {
    pub seed: u64,
}

#[derive(Bundle)]
pub struct AsteroidBundle {
    astre_bundle: AstreBundle,
    asteroid: Asteroid,
    loader: HandleLoaderBundle<MaterialLoader<StarMaterial>>,
}

pub fn build_asteroid_belt(c: &mut ChildBuilder) {
    let mut rng = thread_rng();

    let radius: f32 = rng.gen_range(1000.0..100_000.0);
    let nb_asteroids = rng.gen_range(10..100);

    for i in 0..nb_asteroids {
        let angle = (i as f32 / nb_asteroids as f32) * 2. * PI;

        let local_radius = rng.gen_range(0.8..1.2);

        let position = Vec2::new(
            radius * local_radius * angle.cos(),
            radius * local_radius * angle.sin(),
        );

        build_asteroid(c, position);
    }
}

fn build_asteroid(c: &mut ChildBuilder, position: Vec2) {
    let seed = random::<u64>();
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let avg_radius = rng.gen_range(100.0..1000.0);

    let transform = Transform::from_translation(position.extend(0.));

    let rotation_direction =
        Vec2::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)).normalize();

    let rotation_speed = rng.gen_range(0.001..=0.01);

    let composition = ElementOnAstre::random_elements(
        rng.gen_range(1..=ELEMENTS.len()) as u32,
        rng.gen_range(1000..=10_000),
        &[ElementState::Solid],
    );

    let color = ElementOnAstre::get_color(&composition);

    let material = StarMaterial {
        color,
        seed: rng.gen::<f32>() * 1000.,
        rotation: rotation_direction * rotation_speed,
    };

    c.spawn(AsteroidBundle {
        astre_bundle: AstreBundle::new(avg_radius, 0.0, composition),
        asteroid: Asteroid { seed },
        loader: HandleLoaderBundle {
            loader: MaterialLoader {
                material,
                mesh_type: MeshType::RandomPolygon(seed, avg_radius),
            },
            transform,
            ..default()
        },
    });
}

pub fn random_polygon(seed: u64, avg_radius: f32) -> Mesh {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let num_vertices = rng.gen_range(5..15);
    let irregularity = rng.gen_range(0.0..1.0);
    let spikiness = rng.gen_range(0.0..1.0);

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
        PrimitiveTopology::TriangleList, // Changed to TriangleList
        RenderAssetUsages::RENDER_WORLD,
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

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; points.len()]);

    mesh
}
