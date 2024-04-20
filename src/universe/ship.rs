use bevy::prelude::*;

use crate::{universe::DockableOnAstre, HandleLoaderBundle, SpriteLoader};

pub const SHIP_Z: f32 = 100.;
const SHIP_SCALE: f32 = 1.8;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Ship {
    speed: Vec2,
    max_speed: f32,
    thrust: f32,
}

impl Ship {
    #[inline]
    pub fn speed(&self) -> Vec2 {
        self.speed
    }
}

#[derive(Component)]
pub struct ShipSprite;

pub fn build_ship(c: &mut ChildBuilder) {
    let position = Vec2::new(0., 0.);

    // Ship is just a SpatialBundle
    c.spawn((
        Ship {
            speed: Vec2::new(0., 0.),
            max_speed: 50.,
            thrust: 3000.,
        },
        DockableOnAstre::default(),
        SpatialBundle {
            transform: Transform::from_translation(position.extend(SHIP_Z))
                .with_scale(Vec3::splat(SHIP_SCALE)),
            ..default()
        },
    ));
}

pub fn spawn_ship_sprite(mut commands: Commands, q_ship: Query<Entity, Added<Ship>>) {
    let Some(ship) = q_ship.iter().next() else {
        return;
    };

    commands.entity(ship).with_children(|c| {
        // Ship sprite as a child of ship, so we can rotate the sprite without rotating camera
        c.spawn((
            ShipSprite,
            HandleLoaderBundle {
                loader: SpriteLoader {
                    texture_path: "sprites/ship.png".to_string(),
                    ..default()
                },
                ..default()
            },
        ));
    });
}

pub fn update_ship(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_ship: Query<(&mut Ship, &mut Transform, &DockableOnAstre), Without<ShipSprite>>,
    mut q_ship_sprite: Query<&mut Transform, With<ShipSprite>>,
) {
    for (mut ship, mut transform, dockable) in q_ship.iter_mut() {
        let mut movement = Vec2::new(0., 0.);

        if keyboard_input.pressed(KeyCode::Space) {
            ship.speed *= 0.9;
        }

        if keyboard_input.any_pressed(vec![KeyCode::ArrowLeft, KeyCode::KeyA]) {
            movement.x -= 1.;
        }
        if keyboard_input.any_pressed(vec![KeyCode::ArrowRight, KeyCode::KeyD]) {
            movement.x += 1.;
        }
        if keyboard_input.any_pressed(vec![KeyCode::ArrowUp, KeyCode::KeyW]) {
            movement.y += 1.;
        }
        if keyboard_input.any_pressed(vec![KeyCode::ArrowDown, KeyCode::KeyS]) {
            movement.y -= 1.;
        }

        let acceleration = movement * ship.thrust;

        let max_speed = ship.max_speed;

        ship.speed += (acceleration * time.delta_seconds()).clamp_length_max(max_speed);

        if dockable.is_docked() {
            ship.speed *= 0.99;
        }

        if ship.speed.length() < 0.001 {
            ship.speed = Vec2::ZERO;
        }

        transform.translation.x += ship.speed.x * time.delta_seconds();
        transform.translation.y += ship.speed.y * time.delta_seconds();

        // Sprite rotation
        if let Some(mut sprite_transform) = q_ship_sprite.iter_mut().next() {
            if ship.speed == Vec2::ZERO {
                sprite_transform.rotation = Quat::from_rotation_z(0.);
            } else {
                sprite_transform.rotation =
                    Quat::from_rotation_z(-ship.speed.angle_between(Vec2::Y));
            }
        }
    }
}
