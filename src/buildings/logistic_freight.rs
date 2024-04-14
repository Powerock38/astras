use bevy::{prelude::*, utils::Uuid};
use bevy_mod_picking::prelude::*;

use crate::{
    items::{Inventory, LogisticJourney, LogisticProvider, LogisticRequest, LogisticScope},
    ui::{spawn_cargo_shuttle_ui, spawn_interplanetary_freighter_ui},
};

const RANGE: f32 = 10.0;
const SPEED: f32 = 1000.0;

#[derive(Bundle)]
pub struct LogisticFreightBundle {
    logistic_freight: LogisticFreight,
    inventory: Inventory,
    pointer_event: On<Pointer<Click>>,
}

impl LogisticFreightBundle {
    pub fn new_planet() -> Self {
        Self {
            logistic_freight: LogisticFreight {
                uuid: Uuid::new_v4(),
                cooldown: Timer::from_seconds(1.0, TimerMode::Repeating),
                max_amount_per_transfer: 100,
                journey: None,
                scope: LogisticScope::Planet,
            },
            inventory: Inventory::new(10_000),
            pointer_event: On::<Pointer<Click>>::run(spawn_cargo_shuttle_ui),
        }
    }

    pub fn new_solar_system() -> Self {
        Self {
            logistic_freight: LogisticFreight {
                uuid: Uuid::new_v4(),
                cooldown: Timer::from_seconds(1.0, TimerMode::Repeating),
                max_amount_per_transfer: 100,
                journey: None,
                scope: LogisticScope::SolarSystem,
            },
            inventory: Inventory::new(100_000),
            pointer_event: On::<Pointer<Click>>::run(spawn_interplanetary_freighter_ui),
        }
    }
}

pub type LogisticJourneyWithTarget = (LogisticJourney, Option<Vec3>); // (journey, move_target)

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct LogisticFreight {
    uuid: Uuid,
    cooldown: Timer,
    max_amount_per_transfer: u32,
    journey: Option<LogisticJourneyWithTarget>,
    scope: LogisticScope,
}

impl LogisticFreight {
    #[inline]
    pub fn logistic_journey(&self) -> Option<&LogisticJourney> {
        self.journey.as_ref().map(|(journey, _)| journey)
    }
}

/*
tick:

set journey
    => search for a requester in the same scope
    => search for a compatible provider

journey
    => if logistic_freight inventory CAN'T fullfill requester's request, go to provider
    => get items

    => if logistic_freight inventory CAN partially fullfill requester's request, go to requester
    => give items
*/

pub fn update_logistic_freights(
    time: Res<Time>,
    mut q_logistic_freights: Query<(
        Entity,
        &mut LogisticFreight,
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
        Without<LogisticFreight>,
    >,
    mut q_providers: Query<
        (
            Entity,
            &mut LogisticProvider,
            &Parent,
            &Transform,
            &mut Inventory,
        ),
        (Without<LogisticRequest>, Without<LogisticFreight>),
    >,
) {
    for (freight_entity, mut freight, parent, mut transform, mut inventory) in
        q_logistic_freights.iter_mut()
    {
        if freight.cooldown.tick(time.delta()).finished() {
            // If we already have a journey
            if let Some((journey, move_target)) = &mut freight.journey {
                if let Ok((_, logistic_request, _, requester_transform, mut requester_inventory)) =
                    q_requesters.get_mut(journey.requester())
                {
                    if logistic_request.id() == journey.request_id() {
                        println!("{:?} {:?}", journey, logistic_request);

                        if logistic_request.compute_fulfillment_percentage(&inventory) > 0 {
                            // If freight inventory can (partially) fullfill requester's request, go to requester
                            let target = requester_transform.translation;
                            *move_target = Some(target);

                            if is_target_reached(&transform, target) {
                                // transfer request's items to requester
                                *move_target = None;

                                //TODO: do not transfer all items at once
                                for (item_id, &quantity) in logistic_request.items() {
                                    let q = inventory.transfer_to(
                                        &mut requester_inventory,
                                        item_id.to_string(),
                                        quantity.min(freight.max_amount_per_transfer),
                                    );

                                    println!("Transferred {} {}", q, item_id);
                                }
                                //TODO: if q = 0, requester is full: go to provider OR go to new requester?
                            }
                        } else {
                            // If freight inventory can't fullfill requester's request, go to provider
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
                                            item_id.to_string(),
                                            quantity.min(freight.max_amount_per_transfer),
                                        );

                                        println!("Transferred {} {}", q, item_id);
                                    }
                                }
                            } else {
                                // Provider doesn't exist anymore
                                println!("Provider {:?} doesn't exist anymore", journey.provider());
                                freight.journey = None;
                            }
                        }
                    } else {
                        // Request changed
                        println!("Request changed");
                        freight.journey = None;
                    }
                } else {
                    // Journey requester doesn't exist anymore
                    println!("Requester {:?} doesn't exist anymore", journey.requester());
                    freight.journey = None;
                }
            } else {
                // Search for a requester in the same scope, with the minimum number of freights

                let mut requester = None;
                let mut best_request_nb_freights = usize::MAX;
                for (requester_entity, logistic_request, requester_parent, _, _) in
                    q_requesters.iter_mut()
                {
                    let in_scope = &freight.scope == logistic_request.scope()
                        && match freight.scope {
                            LogisticScope::Planet => requester_parent.get() == parent.get(),
                            LogisticScope::SolarSystem => true,
                            LogisticScope::Interstellar => true,
                        };

                    if in_scope {
                        if logistic_request.freights.len() < best_request_nb_freights {
                            best_request_nb_freights = logistic_request.freights.len();
                            requester = Some((requester_entity, logistic_request));
                        }
                    }
                }

                println!("Requester: {:?}", requester);

                // If we found a requester...
                if let Some((requester_entity, mut logistic_request)) = requester {
                    // ...search for a compatible provider in the same scope,
                    // with the minimum number of freights
                    // and the best fulfillment score of the request

                    let mut provider = None;
                    let mut best_provider_fulfillment_score = 0;
                    let mut best_provider_nb_freights = usize::MAX;
                    for (
                        provider_entity,
                        logistic_provider,
                        provider_parent,
                        _,
                        provider_inventory,
                    ) in q_providers.iter_mut()
                    {
                        let in_scope = &freight.scope == logistic_provider.scope()
                            && match freight.scope {
                                LogisticScope::Planet => provider_parent.get() == parent.get(),
                                LogisticScope::SolarSystem => true,
                                LogisticScope::Interstellar => true,
                            };

                        if in_scope {
                            let fulfillment_score = logistic_request
                                .compute_fulfillment_percentage(&provider_inventory);

                            if fulfillment_score > best_provider_fulfillment_score {
                                best_provider_fulfillment_score = fulfillment_score;
                                best_provider_nb_freights = logistic_provider.freights.len();
                                provider = Some((provider_entity, logistic_provider));
                            } else if fulfillment_score == best_provider_fulfillment_score
                                && logistic_provider.freights.len() < best_provider_nb_freights
                            {
                                best_provider_nb_freights = logistic_provider.freights.len();
                                provider = Some((provider_entity, logistic_provider));
                            }
                        }
                    }

                    println!("Provider: {:?}", provider);

                    // If we found a provider, set the journey and register freight in LogisticRequest and LogisticProvider
                    if let Some((provider_entity, mut logistic_provider)) = provider {
                        logistic_request.freights.push(freight_entity);
                        logistic_provider.freights.push(freight_entity);

                        freight.journey = Some((
                            LogisticJourney::new(
                                logistic_request.id(),
                                provider_entity,
                                requester_entity,
                            ),
                            None,
                        ));
                    }
                }
            }
        }

        // Move towards target
        if let Some((_, move_target)) = &freight.journey {
            if let Some(target) = move_target {
                let direction = *target - transform.translation;
                let distance = direction.length();

                if distance >= RANGE {
                    let direction = direction / distance;
                    let velocity = direction * SPEED;
                    let distance_per_tick = velocity * time.delta_seconds();

                    if distance_per_tick.length() < distance {
                        transform.translation += distance_per_tick;
                    } else {
                        transform.translation = *target;
                    }

                    transform.rotation = Quat::from_rotation_z(
                        (transform.translation.y - target.y)
                            .atan2(transform.translation.x - target.x)
                            + std::f32::consts::FRAC_PI_2,
                    );
                }
            }
        }
    }
}

fn is_target_reached(transform: &Transform, target: Vec3) -> bool {
    (transform.translation - target).length() < RANGE
}
