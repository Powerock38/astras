use bevy::prelude::*;

use crate::items::{Inventory, LogisticJourney, LogisticProvider, LogisticRequest};

const RANGE: f32 = 10.0;
const SPEED: f32 = 10000.0;

#[derive(Bundle)]
pub struct FreighterBundle {
    pub freighter: Freighter,
    pub inventory: Inventory,
}

#[derive(Component, Default)]
pub struct Freighter {
    cooldown: Timer,
    max_amount_per_transfer: u32,
    journey: Option<LogisticJourney>,
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

    => maybe "ping" requester?
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
            if let Some(journey) = &freighter.journey {
                println!("Journey: {:?}", journey);

                if let Ok((_, request, _, requester_transform, mut requester_inventory)) =
                    q_requesters.get_mut(journey.requester())
                {
                    println!("with request: {:?}", request);

                    if request.can_be_partially_fullfilled_by(&inventory) {
                        // If freighter inventory can (partially) fullfill requester's request, go to requester

                        println!("Can be partially fullfilled");

                        //FIXME: moves only every freighter.cooldown
                        if move_towards(
                            &mut transform,
                            requester_transform.translation,
                            SPEED,
                            &time,
                        ) {
                            // transfer request's items to requester

                            println!("in range of requester");

                            for (item_id, &quantity) in request.items() {
                                let q = inventory.transfer_to(
                                    &mut requester_inventory,
                                    item_id,
                                    quantity.min(freighter.max_amount_per_transfer),
                                );

                                println!("Transferred {} {}", q, item_id);
                            }
                        } else {
                            println!("Still moving towards requester");
                        }
                    } else {
                        // If freighter inventory can't fullfill requester's request, go to provider

                        if let Ok((_, _, provider_transform, mut provider_inventory)) =
                            q_providers.get_mut(journey.provider())
                        {
                            println!("Can't be fullfilled, moving towards provider");

                            //FIXME: moves only every freighter.cooldown
                            if move_towards(
                                &mut transform,
                                provider_transform.translation,
                                SPEED,
                                &time,
                            ) {
                                // transfer request's items from provider

                                println!("in range of provider");

                                for (item_id, &quantity) in request.items() {
                                    let q = provider_inventory.transfer_to(
                                        &mut inventory,
                                        item_id,
                                        quantity.min(freighter.max_amount_per_transfer),
                                    );

                                    println!("Transferred {} {}", q, item_id);
                                }
                            } else {
                                println!("Still moving towards provider");
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
                        freighter.journey =
                            Some(LogisticJourney::new(provider_entity, requester_entity));
                    }
                } else {
                    continue;
                }
            }
        }
    }
}

// Returns true if the target is reached
fn move_towards(transform: &mut Transform, target: Vec3, speed: f32, time: &Time) -> bool {
    let direction = target - transform.translation;
    let distance = direction.length();

    if distance >= RANGE {
        let direction = direction / distance;
        let velocity = direction * speed;
        let distance_per_tick = velocity * time.delta_seconds();

        println!(
            "distance: {} velocity: {:?} distance_per_tick: {:?}",
            distance, velocity, distance_per_tick
        );

        if distance_per_tick.length() < distance {
            transform.translation += distance_per_tick;
        } else {
            transform.translation = target;
        }

        false
    } else {
        true
    }
}
