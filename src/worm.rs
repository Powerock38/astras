use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

use crate::constants::COLORS;

#[derive(Component, Debug)]
pub struct Worm {
    pub size: f32,
    length: u32,
    speed: f32,
    direction: Vec2,
    change_direction_cooldown: Timer,
}

pub fn spawn_worm(
    c: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    size: f32,
    speed: f32,
    length: u32,
    change_direction_every: f32,
) {
    let color = COLORS.choose(&mut rand::thread_rng()).unwrap();
    let material = ColorMaterial::from(color.clone());

    let mesh = shape::Box::new(2. * size, size, 0.);

    let transform = Transform::from_translation(position.extend(0.))
        .with_rotation(Quat::from_rotation_z(PI / 2.));

    let worm = Worm {
        size,
        length,
        speed,
        direction: Vec2::new(1., 0.),
        change_direction_cooldown: Timer::from_seconds(change_direction_every, TimerMode::Once),
    };

    c.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh.into()).into(),
        material: materials.add(material.clone()),
        transform,
        ..default()
    })
    .insert(worm)
    .with_children(|c| {
        for n_segment in 1..length {
            let child_position = Vec2::new(-2. * size * n_segment as f32, 0.);

            let color = COLORS.choose(&mut rand::thread_rng()).unwrap();
            let material = ColorMaterial::from(color.clone());

            let transform = Transform::from_translation(child_position.extend(0.));

            c.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(mesh.into()).into(),
                material: materials.add(material.clone()),
                transform,
                ..default()
            });
        }
    });
}

pub fn update_worms(
    time: Res<Time>,
    mut query: Query<(&mut Worm, &mut Transform, &Children)>,
    mut query_children: Query<&mut Transform, Without<Worm>>,
) {
    for (mut worm, mut transform, segments) in query.iter_mut() {
        if worm.change_direction_cooldown.tick(time.delta()).finished() {
            let clamped_angle = PI / 16.;
            let add_angle = rand::thread_rng().gen_range(0.0..=clamped_angle) - clamped_angle;

            transform.rotate(Quat::from_rotation_z(add_angle));

            worm.direction = transform.local_x().truncate();

            worm.change_direction_cooldown.reset();
        }

        let move_progress = worm.change_direction_cooldown.elapsed().as_secs_f32()
            / worm.change_direction_cooldown.duration().as_secs_f32();

        let speed = worm.speed * move_progress;

        transform.translation += worm.direction.extend(0.) * speed * time.delta_seconds();

        // Wiggle

        let time_factor = time.elapsed_seconds() * 10.;

        for (i, segment) in segments.iter().enumerate() {
            let mut segment_transform = query_children.get_mut(*segment).unwrap();

            let x = (i as f32) / (worm.length as f32);

            segment_transform.translation.y = (PI * x + time_factor).sin() * 100.;
        }
    }
}
