use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use rand::prelude::SliceRandom;

use crate::{
    buildings::PlacingBuilding,
    items::{Inventory, ELEMENTS},
    universe::{Astre, DockableOnAstre, Laser, LaserBundle, LaserMaterial},
    HandleLoaderBundle, MaterialLoader, MeshType, SpriteLoader,
};

pub const SHIP_Z: f32 = 100.;

const SHIP_INVENTORY_SIZE: u32 = 100_000;

pub const SHIP_ACTION_RANGE: f32 = 5000.;

const MINING_COOLDOWN: f32 = 0.5;
const MINING_AMOUNT_PER_TICK: u32 = 10;
const MINING_LASER_WIDTH: f32 = 100.;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Ship {
    speed: Vec2,
    max_speed: f32,
    thrust: f32,
    mining_cooldown: Timer,
    mining_amount_per_tick: u32,
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
        Name::new("Ship"),
        Ship {
            speed: Vec2::new(0., 0.),
            max_speed: 50.,
            thrust: 3000.,
            mining_cooldown: Timer::from_seconds(MINING_COOLDOWN, TimerMode::Once),
            mining_amount_per_tick: MINING_AMOUNT_PER_TICK,
        },
        DockableOnAstre::default(),
        SpatialBundle {
            transform: Transform::from_translation(position.extend(SHIP_Z)),
            ..default()
        },
        Inventory::new(SHIP_INVENTORY_SIZE),
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
    for (mut ship, mut transform, dockable) in &mut q_ship {
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

pub fn update_ship_mining(
    mut commands: Commands,
    placing_building: Option<Res<PlacingBuilding>>,
    listener: Listener<Pointer<Down>>,
    mut q_ship: Query<(Entity, &Ship, &GlobalTransform, &mut Inventory)>,
    mut q_astres: Query<&mut Inventory, (With<Astre>, Without<Ship>)>,
) {
    let Some((ship_entity, ship, transform, mut inventory)) = q_ship.iter_mut().next() else {
        return;
    };

    if placing_building.is_some() {
        return;
    }

    let astre_entity = listener.listener();

    // TODO ship.mining_cooldown.tick(time.delta()).finished() ; ship.mining_cooldown.reset();

    if let Ok(mut astre_inventory) = q_astres.get_mut(astre_entity) {
        if let Some(position) = listener.hit.position {
            let position = position.truncate();
            let ship_position = transform.translation().truncate();

            if position.distance(ship_position) < SHIP_ACTION_RANGE {
                let mut rng = rand::thread_rng();
                let item_ids = astre_inventory.all_ids();
                let random_item_id =
                    item_ids.choose_weighted(&mut rng, |id| astre_inventory.quantity(id));

                if let Ok(item_id) = random_item_id {
                    let quantity = astre_inventory
                        .quantity(item_id)
                        .min(ship.mining_amount_per_tick);

                    astre_inventory.transfer_to(&mut inventory, item_id.to_string(), quantity);

                    // Laser beam
                    let color = ELEMENTS.get(item_id).map_or(Color::WHITE, |e| e.color);

                    let relative_position = ship_position - position;
                    let angle = relative_position.y.atan2(relative_position.x);

                    commands.entity(ship_entity).with_children(|c| {
                        c.spawn(LaserBundle {
                            laser: Laser::new(0.5),
                            loader: HandleLoaderBundle {
                                loader: MaterialLoader {
                                    mesh_type: MeshType::Rectangle(
                                        Vec2::ZERO,
                                        Vec2::new(relative_position.length(), MINING_LASER_WIDTH),
                                    ),
                                    material: LaserMaterial::new(color),
                                },
                                transform: Transform::from_translation(
                                    (-relative_position / 2.0).extend(-0.1),
                                )
                                .with_rotation(Quat::from_rotation_z(angle)),
                                ..default()
                            },
                        });
                    });
                }
            }
        }
    }
}
