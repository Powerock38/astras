use bevy::prelude::*;
use rand::seq::IndexedRandom;

use crate::{
    MaterialLoader, MeshType, SpriteLoader,
    buildings::PlacingBuilding,
    data::ELEMENTS,
    items::{ElementState, Inventory},
    ui::NotificationEvent,
    universe::{Astre, DockableOnAstre, Laser, LaserMaterial},
};

pub const SHIP_Z: f32 = 100.;

const SHIP_INVENTORY_SIZE: u32 = 100_000;

pub const SHIP_ACTION_RANGE: f32 = 5000.;

const MINING_COOLDOWN: f32 = 0.5;
const MINING_AMOUNT_PER_TICK: u32 = 10;
const MINING_LASER_WIDTH: f32 = 100.;

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Ship {
    speed: Vec2,
    max_speed: f32,
    thrust: f32,
    mining_cooldown: Timer,
    mining_amount_per_tick: u32,
}

impl Ship {
    pub fn speed(&self) -> Vec2 {
        self.speed
    }
}

#[derive(Component)]
pub struct ShipSprite;

pub fn build_ship() -> impl Bundle {
    let position = Vec2::new(0., 0.);

    (
        Name::new("Ship"),
        Ship {
            speed: Vec2::new(0., 0.),
            max_speed: 50.,
            thrust: 3000.,
            mining_cooldown: Timer::from_seconds(MINING_COOLDOWN, TimerMode::Once),
            mining_amount_per_tick: MINING_AMOUNT_PER_TICK,
        },
        DockableOnAstre::default(),
        Transform::from_translation(position.extend(SHIP_Z)),
        Visibility::default(),
        Inventory::new(SHIP_INVENTORY_SIZE),
    )
}

pub fn spawn_ship_sprite(mut commands: Commands, ship: Single<Entity, Added<Ship>>) {
    commands.entity(*ship).with_children(|c| {
        // Ship sprite as a child of ship, so we can rotate the sprite without rotating camera
        c.spawn((
            ShipSprite,
            SpriteLoader {
                texture_path: "sprites/ship.png".to_string(),
                ..default()
            },
        ));
    });
}

pub fn update_ship(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    q_ship: Single<(&mut Ship, &mut Transform, &DockableOnAstre), Without<ShipSprite>>,
    mut ship_sprite_transform: Single<&mut Transform, With<ShipSprite>>,
) {
    let (mut ship, mut transform, dockable) = q_ship.into_inner();

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

    ship.speed += (acceleration * time.delta_secs()).clamp_length_max(max_speed);

    if dockable.on_astre {
        ship.speed *= 0.99;
    }

    if ship.speed.length() < 0.001 {
        ship.speed = Vec2::ZERO;
    }

    transform.translation.x += ship.speed.x * time.delta_secs();
    transform.translation.y += ship.speed.y * time.delta_secs();

    // Sprite rotation
    if ship.speed == Vec2::ZERO {
        ship_sprite_transform.rotation = Quat::from_rotation_z(0.);
    } else {
        ship_sprite_transform.rotation = Quat::from_rotation_z(-ship.speed.angle_to(Vec2::Y));
    }
}

pub fn update_ship_mining(
    pointer_click: On<Pointer<Click>>,
    mut commands: Commands,
    placing_building: Option<Res<PlacingBuilding>>,
    q_ship: Single<(Entity, &Ship, &GlobalTransform, &mut Inventory)>,
    mut q_astres: Query<(&Astre, &mut Inventory, &GlobalTransform), Without<Ship>>,
) {
    let (ship_entity, ship, transform, mut inventory) = q_ship.into_inner();

    if placing_building.is_some() {
        return;
    }

    // TODO ship.mining_cooldown.tick(time.delta()).finished() ; ship.mining_cooldown.reset();

    if let Ok((astre, mut astre_inventory, astre_global_transform)) =
        q_astres.get_mut(pointer_click.entity)
        && let Some(position) = pointer_click.hit.position
    {
        let position = position.truncate();
        let ship_position = transform.translation().truncate();
        let astre_position = astre_global_transform.translation().truncate();

        if position.distance(astre_position) < astre.atmosphere_radius()
            && position.distance(ship_position) < SHIP_ACTION_RANGE
        {
            let atmosphere_mining = position.distance(astre_position) > astre.surface_radius();

            let item_ids = astre_inventory
                .all_ids()
                .iter()
                .filter(|id| {
                    ELEMENTS
                        .get(*id)
                        .is_some_and(|e| !atmosphere_mining || e.state == ElementState::Gas)
                })
                .copied()
                .collect::<Vec<_>>();

            let mut rng = rand::rng();
            let random_item_id =
                item_ids.choose_weighted(&mut rng, |id| astre_inventory.quantity(*id));

            if let Ok(item_id) = random_item_id {
                let quantity = astre_inventory
                    .quantity(*item_id)
                    .min(ship.mining_amount_per_tick);

                astre_inventory.transfer_to(&mut inventory, *item_id, quantity);

                // Laser beam
                let color = ELEMENTS
                    .get(item_id)
                    .map_or(Color::WHITE.into(), |e| e.color.into());

                let relative_position = ship_position - position;
                let angle = relative_position.y.atan2(relative_position.x);

                commands.entity(ship_entity).with_children(|c| {
                    c.spawn((
                        Laser::new(0.5),
                        MaterialLoader {
                            mesh_type: MeshType::Rectangle(
                                Vec2::ZERO,
                                Vec2::new(relative_position.length(), MINING_LASER_WIDTH),
                            ),
                            material: LaserMaterial::new(color),
                        },
                        Transform::from_translation((-relative_position / 2.0).extend(-0.1))
                            .with_rotation(Quat::from_rotation_z(angle)),
                    ));
                });

                let item = item_id.data();

                commands.trigger(NotificationEvent(format!(
                    "Mined {} (x{quantity})",
                    item.name
                )));
            }
        }
    }
}
