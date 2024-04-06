use bevy::prelude::*;

use crate::items::{Inventory, LogisticJourney, LogisticProvider, LogisticRequest};

const RANGE: f32 = 10.0;
const SPEED: f32 = 1000.0;

#[derive(Bundle)]
pub struct FreighterBundle {
    pub freighter: Freighter,
    pub inventory: Inventory,
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

#[derive(Component, Default)]
pub struct Freighter {
    cooldown: Timer,
    max_amount_per_transfer: u32,
    journey: Option<(LogisticJourney, Option<Vec3>)>, // (journey, move_target)
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
    mut q_freighters: Query<(
        Entity,
        &mut Freighter,
        &Parent,
        &mut Transform,
        &mut Inventory,
    )>,
    mut q_requesters: Query<
        (
            Entity,
            &mut LogisticRequest,
            &Parent,
            &Transform,
            &mut Inventory,
        ),
        Without<Freighter>,
    >,
    mut q_providers: Query<
        (
            Entity,
            &mut LogisticProvider,
            &Parent,
            &Transform,
            &mut Inventory,
        ),
        (Without<LogisticRequest>, Without<Freighter>),
    >,
) {
    for (freighter_entity, mut freighter, parent, mut transform, mut inventory) in
        q_freighters.iter_mut()
    {
        if freighter.cooldown.tick(time.delta()).finished() {
            if let Some((journey, move_target)) = &mut freighter.journey {
                if let Ok((_, logistic_request, _, requester_transform, mut requester_inventory)) =
                    q_requesters.get_mut(journey.requester())
                {
                    if logistic_request.id() == journey.request_id() {
                        println!("{:?} {:?}", journey, logistic_request);

                        if logistic_request.compute_fulfillment_percentage(&inventory) > 0 {
                            // If freighter inventory can (partially) fullfill requester's request, go to requester
                            let target = requester_transform.translation;
                            *move_target = Some(target);

                            if is_target_reached(&transform, target) {
                                // transfer request's items to requester
                                *move_target = None;

                                //TODO: do not transfer all items at once
                                for (item_id, &quantity) in logistic_request.items() {
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
                            if let Ok((_, _, _, provider_transform, mut provider_inventory)) =
                                q_providers.get_mut(journey.provider())
                            {
                                let target = provider_transform.translation;
                                *move_target = Some(target);

                                if is_target_reached(&transform, target) {
                                    // transfer request's items from provider
                                    *move_target = None;

                                    //TODO: do not transfer all items at once
                                    for (item_id, &quantity) in logistic_request.items() {
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
                        // Request changed
                        println!("Request changed");
                        freighter.journey = None;
                    }
                } else {
                    // Journey requester doesn't exist anymore
                    println!("Requester {:?} doesn't exist anymore", journey.requester());
                    freighter.journey = None;
                }
            } else {
                // Search for a requester on the same planet, with the minimum number of freighters

                let mut requester = None;
                let mut best_request_nb_freighters = usize::MAX;
                for (requester_entity, logistic_request, requester_parent, _, _) in
                    q_requesters.iter_mut()
                {
                    if requester_parent.get() == parent.get() {
                        if logistic_request.freighters.len() < best_request_nb_freighters {
                            best_request_nb_freighters = logistic_request.freighters.len();
                            requester = Some((requester_entity, logistic_request));
                        }
                    }
                }

                println!("Requester: {:?}", requester);

                // If we found a requester...
                if let Some((requester_entity, mut logistic_request)) = requester {
                    // ...search for a compatible provider on the same planet,
                    // with the minimum number of freighters
                    // and the best fulfillment score of the request

                    let mut provider = None;
                    let mut best_provider_fulfillment_score = 0;
                    let mut best_provider_nb_freighters = usize::MAX;
                    for (
                        provider_entity,
                        logistic_provider,
                        provider_parent,
                        _,
                        provider_inventory,
                    ) in q_providers.iter_mut()
                    {
                        // if provider is on same planet and can fulfill the request
                        if provider_parent.get() == parent.get() {
                            let fulfillment_score = logistic_request
                                .compute_fulfillment_percentage(&provider_inventory);

                            if fulfillment_score > best_provider_fulfillment_score {
                                best_provider_fulfillment_score = fulfillment_score;
                                best_provider_nb_freighters = logistic_provider.freighters.len();
                                provider = Some((provider_entity, logistic_provider));
                            } else if fulfillment_score == best_provider_fulfillment_score
                                && logistic_provider.freighters.len() < best_provider_nb_freighters
                            {
                                best_provider_nb_freighters = logistic_provider.freighters.len();
                                provider = Some((provider_entity, logistic_provider));
                            }
                        }
                    }

                    println!("Provider: {:?}", provider);

                    // If we found a provider, set the journey and register freighter in LogisticRequest and LogisticProvider
                    if let Some((provider_entity, mut logistic_provider)) = provider {
                        logistic_request.freighters.push(freighter_entity);
                        logistic_provider.freighters.push(freighter_entity);

                        freighter.journey = Some((
                            LogisticJourney::new(
                                logistic_request.id(),
                                provider_entity,
                                requester_entity,
                            ),
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
