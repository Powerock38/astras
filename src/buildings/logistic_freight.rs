use bevy::{prelude::*, utils::HashSet};

use crate::items::{
    Inventory, ItemMap, LogisticJourney, LogisticProvider, LogisticRequest, LogisticScope,
};

const RANGE: f32 = 100.0;
const SPEED: f32 = 1000.0;
const LOGISTIC_FREIGHTER_Z: f32 = 0.6;

//TODO: implement Ship following (to move freighters manually)

pub type LogisticJourneyWithTarget = (LogisticJourney, Option<Entity>); // (journey, target)

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Inventory)]
pub struct LogisticFreight {
    cooldown: Timer,
    max_amount_per_transfer: u32,
    journey: Option<LogisticJourneyWithTarget>,
    scope: LogisticScope,
}

impl LogisticFreight {
    pub fn new_planet() -> Self {
        Self {
            scope: LogisticScope::Planet,
            cooldown: Timer::from_seconds(1.0, TimerMode::Repeating),
            max_amount_per_transfer: 100,
            journey: None,
        }
    }

    pub fn new_solar_system() -> Self {
        Self {
            scope: LogisticScope::SolarSystem,
            cooldown: Timer::from_seconds(1.0, TimerMode::Repeating),
            max_amount_per_transfer: 100,
            journey: None,
        }
    }

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

all the mut logic is splitted in observers, in order to avoid conflicting queries / paramset borrowing issues
*/

pub fn update_logistic_freights(
    time: Res<Time>,
    mut commands: Commands,
    mut q_logistic_freights: Query<
        (
            Entity,
            &mut LogisticFreight,
            &Parent,
            &GlobalTransform,
            &Inventory,
        ),
        (Without<LogisticRequest>, Without<LogisticProvider>),
    >,
    q_requesters: Query<(Entity, &LogisticRequest, &Parent, &GlobalTransform)>,
    q_providers: Query<(
        Entity,
        &LogisticProvider,
        &Parent,
        &GlobalTransform,
        &Inventory,
    )>,
    q_parent: Query<&Parent>,
) {
    for (freight_entity, mut freight, parent, transform, inventory) in &mut q_logistic_freights {
        if freight.cooldown.tick(time.delta()).finished() {
            // If we already have a journey
            if let Some((journey, move_target)) = &mut freight.journey {
                if let Ok((requester_entity, logistic_request, _, requester_transform)) =
                    q_requesters.get(journey.requester())
                {
                    if logistic_request.id() == journey.request_id() {
                        println!("{journey:?} {logistic_request:?}");

                        if logistic_request.compute_fulfillment_percentage(inventory) > 0 {
                            // If freight inventory can (partially) fullfill requester's request, go to requester
                            *move_target = Some(requester_entity);

                            if is_target_reached(transform, requester_transform) {
                                // transfer request's items to requester
                                *move_target = None;

                                commands.trigger(FreightInventoryTransfer {
                                    items: logistic_request.items().clone(),
                                    freight: freight_entity,
                                    provider_or_requester: journey.requester(),
                                    is_provider: false,
                                });
                            }
                        } else {
                            // If freight inventory can't fullfill requester's request, go to provider
                            if let Ok((provider_entity, _, _, provider_transform, _)) =
                                q_providers.get(journey.provider())
                            {
                                *move_target = Some(provider_entity);

                                if is_target_reached(transform, provider_transform) {
                                    // transfer request's items from provider
                                    *move_target = None;

                                    commands.trigger(FreightInventoryTransfer {
                                        items: logistic_request.items().clone(),
                                        freight: freight_entity,
                                        provider_or_requester: journey.provider(),
                                        is_provider: true,
                                    });
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
                        commands.trigger(UnregisterFreight(freight_entity));
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
                    .iter()
                    .filter(
                        |(requester_entity, logistic_request, requester_parent, ..)| {
                            &freight.scope == logistic_request.scope()
                                && match freight.scope {
                                    LogisticScope::Planet => requester_parent.get() == parent.get(),
                                    LogisticScope::SolarSystem => {
                                        let freight_ancestors = q_parent
                                            .iter_ancestors(freight_entity)
                                            .collect::<HashSet<_>>();

                                        q_parent
                                            .iter_ancestors(*requester_entity)
                                            .any(|p| freight_ancestors.contains(&p))
                                    }
                                }
                        },
                    )
                    .collect::<Vec<_>>();

                requesters
                    .sort_by(|(_, a, ..), (_, b, ..)| a.freights.len().cmp(&b.freights.len()));

                for (requester_entity, logistic_request, ..) in requesters {
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
                    ) in &q_providers
                    {
                        if provider_entity == requester_entity {
                            continue;
                        }

                        let in_scope = &freight.scope == logistic_provider.scope()
                            && match freight.scope {
                                LogisticScope::Planet => provider_parent.get() == parent.get(),
                                LogisticScope::SolarSystem => {
                                    let freight_ancestors = q_parent
                                        .iter_ancestors(freight_entity)
                                        .collect::<HashSet<_>>();

                                    q_parent
                                        .iter_ancestors(provider_entity)
                                        .any(|p| freight_ancestors.contains(&p))
                                }
                            };

                        if in_scope {
                            let fulfillment_score =
                                logistic_request.compute_fulfillment_percentage(provider_inventory);

                            if fulfillment_score > best_provider_fulfillment_score
                                || (fulfillment_score == best_provider_fulfillment_score
                                    && logistic_provider.freights.len() < best_provider_nb_freights)
                            {
                                best_provider_fulfillment_score = fulfillment_score;
                                best_provider_nb_freights = logistic_provider.freights.len();
                                provider = Some(provider_entity);
                            }
                        }
                    }

                    println!(
                        "Request {logistic_request:?} for {requester_entity:?} | Provider: {provider:?}"
                    );

                    // If we found a provider, set the journey and register freight in LogisticRequest and LogisticProvider
                    if let Some(provider_entity) = provider {
                        commands.trigger(RegisterFreight {
                            freight: freight_entity,
                            requester: requester_entity,
                            provider: provider_entity,
                        });

                        break;
                    }
                }
            }
        }
    }
}

pub fn update_logistic_freights_movement(
    time: Res<Time>,
    q_global_transforms: Query<&GlobalTransform>,
    mut q_logistic_freights: Query<(&LogisticFreight, &Parent, &mut Transform)>,
) {
    for (freight, parent, mut transform) in &mut q_logistic_freights {
        // Move towards target
        if let Some((_, Some(target))) = &freight.journey {
            let target_global_transform = q_global_transforms.get(*target).unwrap();
            let parent_global_transform = q_global_transforms.get(parent.get()).unwrap();

            let target = target_global_transform
                .reparented_to(parent_global_transform)
                .translation
                .truncate();

            let direction = target - transform.translation.truncate();
            let distance = direction.length();

            if distance >= RANGE {
                let direction = direction / distance;
                let velocity = direction * SPEED;
                let distance_per_tick = velocity * time.delta_secs();

                if distance_per_tick.length() < distance {
                    transform.translation.x += distance_per_tick.x;
                    transform.translation.y += distance_per_tick.y;
                } else {
                    transform.translation.x = target.x;
                    transform.translation.y = target.y;
                    println!("Target reached {target:?}");
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

fn is_target_reached(a: &GlobalTransform, b: &GlobalTransform) -> bool {
    (a.translation().truncate() - b.translation().truncate()).length() < RANGE
}

#[derive(Event)]
pub struct RegisterFreight {
    freight: Entity,
    requester: Entity,
    provider: Entity,
}

pub fn observe_register_freight(
    trigger: Trigger<RegisterFreight>,
    mut q_logistic_freights: Query<
        &mut LogisticFreight,
        (Without<LogisticRequest>, Without<LogisticProvider>),
    >,
    mut paramset: ParamSet<(Query<&mut LogisticRequest>, Query<&mut LogisticProvider>)>,
) {
    let mut freight = q_logistic_freights.get_mut(trigger.freight).unwrap();

    {
        let mut q_logistic_provider = paramset.p1();
        let mut logistic_provider = q_logistic_provider.get_mut(trigger.provider).unwrap();
        logistic_provider.freights.push(trigger.freight);
    }

    let mut q_logistic_request = paramset.p0();
    let mut logistic_request = q_logistic_request.get_mut(trigger.requester).unwrap();
    logistic_request.freights.push(trigger.freight);

    freight.journey = Some((
        LogisticJourney::new(logistic_request.id(), trigger.provider, trigger.requester),
        None,
    ));
}

#[derive(Event)]
pub struct UnregisterFreight(Entity);

pub fn observe_unregister_freight(
    trigger: Trigger<UnregisterFreight>,
    mut q_logistic_freights: Query<
        &mut LogisticFreight,
        (Without<LogisticRequest>, Without<LogisticProvider>),
    >,
    mut paramset: ParamSet<(Query<&mut LogisticRequest>, Query<&mut LogisticProvider>)>,
) {
    let freight_entity = trigger.0;
    let mut freight = q_logistic_freights.get_mut(freight_entity).unwrap();

    if let Some((journey, _)) = freight.journey.take() {
        if let Ok(mut logistic_request) = paramset.p0().get_mut(journey.requester()) {
            logistic_request.freights.retain(|&f| f != freight_entity);
        }

        if let Ok(mut logistic_provider) = paramset.p1().get_mut(journey.provider()) {
            logistic_provider.freights.retain(|&f| f != freight_entity);
        }

        freight.journey = None;
    }
}

#[derive(Event)]
pub struct FreightInventoryTransfer {
    items: ItemMap,
    freight: Entity,
    provider_or_requester: Entity,
    is_provider: bool, // true = from provider to freight, false = from freight to requester
}

pub fn observe_freight_inventory_transfer(
    trigger: Trigger<FreightInventoryTransfer>,
    mut commands: Commands,
    mut q_freight: Query<(&LogisticFreight, &mut Inventory)>,
    mut q_providers_or_requesters: Query<&mut Inventory, Without<LogisticFreight>>,
) {
    let (freight, freight_inventory) = q_freight.get_mut(trigger.freight).unwrap();
    let other_inventory = q_providers_or_requesters
        .get_mut(trigger.provider_or_requester)
        .unwrap();

    let (mut from, mut to) = if trigger.is_provider {
        (other_inventory, freight_inventory)
    } else {
        (freight_inventory, other_inventory)
    };

    // Try transfering some items
    for (&item_id, &quantity) in &trigger.items {
        let q = from.transfer_to(
            &mut to,
            item_id,
            quantity.min(freight.max_amount_per_transfer),
        );

        if q != 0 {
            println!("Transferred {q} {item_id:?}");
            return;
        }
    }

    // We didn't transfer any items (didn't reach return above), unregister freight
    commands.trigger(UnregisterFreight(trigger.freight));
}
