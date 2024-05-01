use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{universe::SHIP_Z, HandleLoaderBundle, SpriteLoader};

const WORM_Z: f32 = SHIP_Z - 2.0;
const WORM_Z_DELTA: f32 = 0.001;
const SEGMENT_WIDTH: f32 = 80.;
const HEAD_WIDTH: f32 = 160.;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Worm {
    length: u32,
    speed: f32,
    direction: Vec2,
    change_direction_cooldown: Timer,
    seed: f32,
    wiggle_amplitude: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct WormSegment;

pub fn build_worm(c: &mut ChildBuilder, position: Vec2) {
    let mut rng = thread_rng();

    let size = rng.gen_range(1. ..=10.);
    let length = rng.gen_range(5..=50);
    let speed = rng.gen_range(100. ..=1000.);
    let wiggle_amplitude = rng.gen_range(5. ..=15.);
    let change_direction_every = rng.gen_range(0.1..=3.);

    let color = Color::rgb(
        rng.gen_range(0. ..=1.),
        rng.gen_range(0. ..=1.),
        rng.gen_range(0. ..=1.),
    );

    let transform =
        Transform::from_translation(position.extend(WORM_Z - (length as f32) * WORM_Z_DELTA))
            .with_scale(Vec3::splat(size));

    c.spawn((
        HandleLoaderBundle {
            loader: SpriteLoader {
                texture_path: "sprites/worm_head.png".to_string(),
                color,
                ..default()
            },
            transform,
            ..default()
        },
        Worm {
            length,
            speed,
            direction: Vec2::new(0., 0.),
            change_direction_cooldown: Timer::from_seconds(change_direction_every, TimerMode::Once),
            seed: rng.gen(),
            wiggle_amplitude,
        },
    ))
    .with_children(|c| {
        for n_segment in 0..length {
            let segment_position = Vec2::new(-(SEGMENT_WIDTH * n_segment as f32 + HEAD_WIDTH), 0.0);

            let transform = Transform::from_translation(
                segment_position.extend(n_segment as f32 * WORM_Z_DELTA),
            );

            c.spawn((
                WormSegment,
                HandleLoaderBundle {
                    loader: SpriteLoader {
                        texture_path: "sprites/worm_segment.png".to_string(),
                        color,
                        ..default()
                    },
                    transform,
                    ..default()
                },
            ));
        }
    });
}

pub fn update_worms(
    time: Res<Time>,
    mut q_worms: Query<(&mut Worm, &mut Transform, &Children)>,
    mut q_segments: Query<&mut Transform, (With<WormSegment>, Without<Worm>)>,
) {
    for (mut worm, mut transform, segments) in q_worms.iter_mut() {
        if worm.change_direction_cooldown.tick(time.delta()).finished() {
            let clamped_angle = PI / 1024.;
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
        let mut last_y = 0.0;
        for (i, segment) in segments.iter().enumerate() {
            let mut segment_transform = q_segments.get_mut(*segment).unwrap();

            let x = (i as f32) / (worm.length as f32) * 2.0 * PI
                + time.elapsed_seconds()
                + worm.seed * 100.0;

            segment_transform.translation.y = x.sin() * i as f32 * worm.wiggle_amplitude;

            // Set segment rotation to match normal of the curve
            let angle = -(segment_transform.translation.y - last_y).atan2(SEGMENT_WIDTH);
            segment_transform.rotation = Quat::from_rotation_z(angle);
            last_y = segment_transform.translation.y;
        }
    }
}
