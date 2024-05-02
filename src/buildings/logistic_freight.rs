use bevy::{prelude::*, utils::Uuid};

use crate::items::{Inventory, LogisticJourney, LogisticProvider, LogisticRequest, LogisticScope};

const RANGE: f32 = 10.0;
const SPEED: f32 = 1000.0;
const LOGISTIC_FREIGHTER_Z: f32 = 0.6;

//TODO: implement Ship following (to move freighters manually)

#[derive(Bundle)]
pub struct LogisticFreightBundle {
    logistic_freight: LogisticFreight,
    inventory: Inventory,
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
        }
    }
}

pub type LogisticJourneyWithTarget = (LogisticJourney, Option<Vec2>); // (journey, move_target)

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

    #[inline]
    pub fn scope(&self) -> &LogisticScope {
        &self.scope
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
        &mut q_logistic_freights
    {
        if freight.cooldown.tick(time.delta()).finished() {
            // If we already have a journey
            if let Some((journey, move_target)) = &mut freight.journey {
                if let Ok((_, logistic_request, _, requester_transform, mut requester_inventory)) =
                    q_requesters.get_mut(journey.requester())
                {
                    if logistic_request.id() == journey.request_id() {
                        println!("{journey:?} {logistic_request:?}");

                        let mut unregister_freight = false;

                        if logistic_request.compute_fulfillment_percentage(&inventory) > 0 {
                            // If freight inventory can (partially) fullfill requester's request, go to requester
                            let target = requester_transform.translation.truncate();
                            *move_target = Some(target);

                            if is_target_reached(&transform, target) {
                                // transfer request's items to requester
                                *move_target = None;

                                // Try transfering some items
                                for (item_id, &quantity) in logistic_request.items() {
                                    let q = inventory.transfer_to(
                                        &mut requester_inventory,
                                        item_id.to_string(),
                                        quantity.min(freight.max_amount_per_transfer),
                                    );

                                    if q != 0 {
                                        println!("Transferred {q} {item_id}");
                                        return; // wait for next tick
                                    }
                                }

                                // We didn't transfer any items (didn't reach return above), unregister freight
                                unregister_freight = true;
                            }
                        } else {
                            // If freight inventory can't fullfill requester's request, go to provider
                            if let Ok((_, _, _, provider_transform, mut provider_inventory)) =
                                q_providers.get_mut(journey.provider())
                            {
                                let target = provider_transform.translation.truncate();
                                *move_target = Some(target);

                                if is_target_reached(&transform, target) {
                                    // transfer request's items from provider
                                    *move_target = None;

                                    // Try transfering some items
                                    for (item_id, &quantity) in logistic_request.items() {
                                        let q = provider_inventory.transfer_to(
                                            &mut inventory,
                                            item_id.to_string(),
                                            quantity.min(freight.max_amount_per_transfer),
                                        );

                                        if q != 0 {
                                            println!("Transferred {q} {item_id}");
                                            return; // wait for next tick
                                        }
                                    }

                                    // We didn't transfer any items (didn't reach return above), unregister freight
                                    unregister_freight = true;
                                }
                            } else {
                                // Provider doesn't exist anymore
                                println!("Provider {:?} doesn't exist anymore", journey.provider());
                                freight.journey = None;
                            }
                        }

                        if unregister_freight {
                            if let Some((journey, _)) = freight.journey.take() {
                                if let Ok((_, mut logistic_request, _, _, _)) =
                                    q_requesters.get_mut(journey.requester())
                                {
                                    logistic_request.freights.retain(|&f| f != freight_entity);
                                }

                                if let Ok((_, mut logistic_provider, _, _, _)) =
                                    q_providers.get_mut(journey.provider())
                                {
                                    logistic_provider.freights.retain(|&f| f != freight_entity);
                                }
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
                // Search for the best requester / provider pair

                // Search for requesters in the same scope, prioritizing the one with the least freights

                let mut requesters = q_requesters
                    .iter_mut()
                    .filter(|(_, logistic_request, requester_parent, ..)| {
                        &freight.scope == logistic_request.scope()
                            && match freight.scope {
                                LogisticScope::Planet => requester_parent.get() == parent.get(),
                                LogisticScope::SolarSystem => true,
                            }
                    })
                    .collect::<Vec<_>>();

                requesters
                    .sort_by(|(_, a, ..), (_, b, ..)| a.freights.len().cmp(&b.freights.len()));

                for (requester_entity, mut logistic_request, ..) in requesters {
                    // Search for a compatible provider in the same scope,
                    // with the minimum number of freights
                    // and the best fulfillment score for the request

                    let mut provider = None;
                    let mut best_provider_fulfillment_score = 1;
                    let mut best_provider_nb_freights = usize::MAX;
                    for (
                        provider_entity,
                        logistic_provider,
                        provider_parent,
                        _,
                        provider_inventory,
                    ) in &mut q_providers
                    {
                        let in_scope = &freight.scope == logistic_provider.scope()
                            && match freight.scope {
                                LogisticScope::Planet => provider_parent.get() == parent.get(),
                                LogisticScope::SolarSystem => true,
                            };

                        if in_scope {
                            let fulfillment_score = logistic_request
                                .compute_fulfillment_percentage(&provider_inventory);

                            if fulfillment_score > best_provider_fulfillment_score
                                || (fulfillment_score == best_provider_fulfillment_score
                                    && logistic_provider.freights.len() < best_provider_nb_freights)
                            {
                                best_provider_fulfillment_score = fulfillment_score;
                                best_provider_nb_freights = logistic_provider.freights.len();
                                provider = Some((provider_entity, logistic_provider));
                            }
                        }
                    }

                    println!(
                        "Request {logistic_request:?} for {requester_entity:?} | Provider: {provider:?}"
                    );

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

                        break;
                    }
                }
            }
        }

        // Move towards target
        if let Some((_, Some(target))) = &freight.journey {
            let direction = *target - transform.translation.truncate();
            let distance = direction.length();

            if distance >= RANGE {
                let direction = direction / distance;
                let velocity = direction * SPEED;
                let distance_per_tick = velocity * time.delta_seconds();

                if distance_per_tick.length() < distance {
                    transform.translation.x += distance_per_tick.x;
                    transform.translation.y += distance_per_tick.y;
                } else {
                    transform.translation.x = target.x;
                    transform.translation.y = target.y;
                }

                transform.translation.z = LOGISTIC_FREIGHTER_Z;

                transform.rotation = Quat::from_rotation_z(
                    (transform.translation.y - target.y).atan2(transform.translation.x - target.x)
                        + std::f32::consts::FRAC_PI_2,
                );
            }
        }
    }
}

fn is_target_reached(transform: &Transform, target: Vec2) -> bool {
    (transform.translation.truncate() - target).length() < RANGE
}
