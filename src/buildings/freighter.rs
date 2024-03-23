use bevy::prelude::*;

use crate::{items::Inventory, Warehouse};

const RANGE: f32 = 100.0;

#[derive(Bundle)]
pub struct FreighterBundle {
    pub freighter: Freighter,
    pub inventory: Inventory,
}

#[derive(Component, Default)]
pub struct Freighter {
    cooldown: Timer,
    source: Option<Entity>,
    destination: Option<Entity>,
    amount_per_transfer: u32,
}

impl Default for FreighterBundle {
    fn default() -> Self {
        Self {
            freighter: Freighter {
                cooldown: Timer::from_seconds(1.0, TimerMode::Repeating),
                ..default()
            },
            inventory: Inventory::new(10_000),
        }
    }
}

pub fn update_freighters(
    time: Res<Time>,
    mut q_freighters: Query<(&mut Freighter, &Parent, &mut Transform, &mut Inventory)>,
    mut q_warehouses: Query<
        (Entity, &Parent, &Transform, &mut Inventory),
        (With<Warehouse>, Without<Freighter>),
    >,
) {
    for (mut freighter, parent, mut transform, mut inventory) in q_freighters.iter_mut() {
        if freighter.cooldown.tick(time.delta()).finished() {
            if let Some(source) = freighter.source {
                // If inventory is empty, go to source
                if inventory.is_empty() {
                    // Check distance to source
                    if let Ok((_, _, source_transform, _)) = q_warehouses.get(source) {
                        let direction = source_transform.translation - transform.translation;
                        let distance = direction.length();

                        // if distance is greater than action range, move towards source
                        if distance > RANGE {
                            let direction = direction / distance;
                            let velocity = direction * 100.0;
                            let distance_per_tick = velocity * time.delta_seconds();
                            if distance_per_tick.length() < distance {
                                transform.translation += distance_per_tick;
                            } else {
                                transform.translation = source_transform.translation;
                            }
                        } else {
                            // else get items from source
                            if let Ok((_, _, _, mut warehouse_inventory)) =
                                q_warehouses.get_mut(source)
                            {
                                if let Some(item_id) = warehouse_inventory.least_quantity_item_id()
                                {
                                    warehouse_inventory.transfer_to(
                                        &mut inventory,
                                        item_id,
                                        freighter.amount_per_transfer,
                                    );
                                }
                            }
                        }
                    }
                }
            } else {
                // Find a source (a warehouse on the same planet)
                for (warehouse_entity, warehouse_parent, _, _) in q_warehouses.iter() {
                    if warehouse_parent.get() == parent.get() {
                        freighter.source = Some(warehouse_entity);
                        break;
                    }
                }
            }
        }
    }
}
