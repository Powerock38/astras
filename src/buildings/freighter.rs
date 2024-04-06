use bevy::prelude::*;

use crate::items::{Inventory, LogisticJourney, LogisticProvider, LogisticRequest};

const RANGE: f32 = 10.0;
const SPEED: f32 = 1000.0;

#[derive(Bundle)]
pub struct FreighterBundle {
    pub freighter: Freighter,
    pub inventory: Inventory,
}

#[derive(Component, Default)]
pub struct Freighter {
    cooldown: Timer,
    max_amount_per_transfer: u32,
    journey: Option<(LogisticJourney, Option<Vec3>)>, // (journey, move_target)
}

impl Default for FreighterBundle {
    fn default() -> Self {
        Self {
            freighter: Freighter {
                cooldown: Timer::from_seconds(1.0, TimerMode::Repeating),
                max_amount_per_transfer: 100,
                journey: None,
            },
            inventory: Inventory::new(10_000),
        }
    }
}

/*
tick:

set journey
    => search for a requester on the same planet
    => search for a compatible provider

journey
    => if freighter inventory CAN'T fullfill requester's request, go to provider
    => get items

    => if freighter inventory CAN partially fullfill requester's request, go to requester
    => give items
*/

pub fn update_freighters(
    time: Res<Time>,
    mut q_freighters: Query<(&mut Freighter, &Parent, &mut Transform, &mut Inventory)>,
    mut q_requesters: Query<
        (
            Entity,
            &LogisticRequest,
            &Parent,
            &Transform,
            &mut Inventory,
        ),
        Without<Freighter>,
    >,
    mut q_providers: Query<
        (Entity, &Parent, &Transform, &mut Inventory),
        (
            With<LogisticProvider>,
            Without<LogisticRequest>,
            Without<Freighter>,
        ),
    >,
) {
    for (mut freighter, parent, mut transform, mut inventory) in q_freighters.iter_mut() {
        if freighter.cooldown.tick(time.delta()).finished() {
            if let Some((journey, move_target)) = &mut freighter.journey {
                if let Ok((_, request, _, requester_transform, mut requester_inventory)) =
                    q_requesters.get_mut(journey.requester())
                {
                    println!("{:?} {:?}", journey, request);

                    if request.can_be_partially_fullfilled_by(&inventory) {
                        // If freighter inventory can (partially) fullfill requester's request, go to requester
                        let target = requester_transform.translation;
                        *move_target = Some(target);

                        if is_target_reached(&transform, target) {
                            // transfer request's items to requester
                            *move_target = None;

                            //TODO: do not transfer all items at once
                            for (item_id, &quantity) in request.items() {
                                let q = inventory.transfer_to(
                                    &mut requester_inventory,
                                    item_id,
                                    quantity.min(freighter.max_amount_per_transfer),
                                );

                                println!("Transferred {} {}", q, item_id);
                            }
                            //TODO: if q = 0, requester is full: go to provider OR go to new requester?
                        }
                    } else {
                        // If freighter inventory can't fullfill requester's request, go to provider
                        if let Ok((_, _, provider_transform, mut provider_inventory)) =
                            q_providers.get_mut(journey.provider())
                        {
                            let target = provider_transform.translation;
                            *move_target = Some(target);

                            if is_target_reached(&transform, target) {
                                // transfer request's items from provider
                                *move_target = None;

                                //TODO: do not transfer all items at once
                                for (item_id, &quantity) in request.items() {
                                    let q = provider_inventory.transfer_to(
                                        &mut inventory,
                                        item_id,
                                        quantity.min(freighter.max_amount_per_transfer),
                                    );

                                    println!("Transferred {} {}", q, item_id);
                                }
                            }
                        } else {
                            // Provider doesn't exist anymore
                            println!("Provider {:?} doesn't exist anymore", journey.provider());
                            freighter.journey = None;
                        }
                    }
                } else {
                    // Journey requester doesn't exist anymore
                    println!("Requester {:?} doesn't exist anymore", journey.requester());
                    freighter.journey = None;
                }
            } else {
                // Search for a requester on the same planet

                // TODO: round robin
                let mut requester = None;
                for (requester_entity, request, requester_parent, _, _) in q_requesters.iter() {
                    if requester_parent.get() == parent.get() {
                        requester = Some((requester_entity, request));
                        break;
                    }
                }

                println!("Requester: {:?}", requester);

                // If we found a requester...
                if let Some((requester_entity, request)) = requester {
                    // ...search for a compatible provider

                    // TODO: round robin
                    let mut provider = None;
                    for (provider_entity, provider_parent, _, provider_inventory) in
                        q_providers.iter()
                    {
                        // if provider is on same planet and can fulfill the request
                        if provider_parent.get() == parent.get()
                            && request.can_be_partially_fullfilled_by(provider_inventory)
                        {
                            provider = Some(provider_entity);
                            break;
                        }
                    }

                    println!("Provider: {:?}", provider);

                    // If we found a provider, set the journey
                    if let Some(provider_entity) = provider {
                        freighter.journey = Some((
                            LogisticJourney::new(provider_entity, requester_entity),
                            None,
                        ));
                    }
                } else {
                    continue;
                }
            }
        }

        // Move towards target
        if let Some((_, move_target)) = &freighter.journey {
            if let Some(target) = move_target {
                move_towards(&mut transform, *target, SPEED, &time);
            }
        }
    }
}

fn is_target_reached(transform: &Transform, target: Vec3) -> bool {
    (transform.translation - target).length() < RANGE
}

fn move_towards(transform: &mut Transform, target: Vec3, speed: f32, time: &Time) {
    let direction = target - transform.translation;
    let distance = direction.length();

    if distance >= RANGE {
        let direction = direction / distance;
        let velocity = direction * speed;
        let distance_per_tick = velocity * time.delta_seconds();

        if distance_per_tick.length() < distance {
            transform.translation += distance_per_tick;
        } else {
            transform.translation = target;
        }

        transform.rotation = Quat::from_rotation_z(
            (transform.translation.y - target.y).atan2(transform.translation.x - target.x)
                + std::f32::consts::FRAC_PI_2,
        );
    }
}
