use bevy::prelude::*;
use rand::prelude::SliceRandom;

use crate::{
    items::{ElementState, Inventory, LogisticProvider, LogisticScope, ELEMENTS},
    universe::Astre,
};

#[derive(Bundle)]
pub struct ExtractorBundle {
    extractor: Extractor,
    inventory: Inventory,
    logistic_provider: LogisticProvider,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Extractor {
    cooldown: Timer,
    amount_per_tick: u32,
    element_state: ElementState,
    cached_item_ids: Option<Vec<String>>,
}

impl ExtractorBundle {
    pub fn new_solid() -> Self {
        Self {
            extractor: Extractor {
                element_state: ElementState::Solid,
                cooldown: Timer::from_seconds(1., TimerMode::Repeating),
                amount_per_tick: 100,
                cached_item_ids: None,
            },
            inventory: Inventory::new(1000),
            logistic_provider: LogisticProvider::new(LogisticScope::Planet),
        }
    }

    pub fn new_liquid() -> Self {
        Self {
            extractor: Extractor {
                element_state: ElementState::Liquid,
                cooldown: Timer::from_seconds(1., TimerMode::Repeating),
                amount_per_tick: 1000,
                cached_item_ids: None,
            },
            inventory: Inventory::new(5000),
            logistic_provider: LogisticProvider::new(LogisticScope::Planet),
        }
    }

    pub fn new_gas() -> Self {
        Self {
            extractor: Extractor {
                element_state: ElementState::Gas,
                cooldown: Timer::from_seconds(1., TimerMode::Repeating),
                amount_per_tick: 500,
                cached_item_ids: None,
            },
            inventory: Inventory::new(10_000),
            logistic_provider: LogisticProvider::new(LogisticScope::Planet),
        }
    }

    pub fn new_plasma() -> Self {
        Self {
            extractor: Extractor {
                element_state: ElementState::Plasma,
                cooldown: Timer::from_seconds(1., TimerMode::Repeating),
                amount_per_tick: 10,
                cached_item_ids: None,
            },
            inventory: Inventory::new(100),
            logistic_provider: LogisticProvider::new(LogisticScope::Planet),
        }
    }
}

pub fn update_extractors(
    time: Res<Time>,
    mut q_extractor: Query<(&mut Extractor, &mut Inventory, &Parent), Without<Astre>>,
    mut q_astre_inventories: Query<&mut Inventory, With<Astre>>,
) {
    for (mut extractor, mut extractor_inventory, parent) in q_extractor.iter_mut() {
        extractor.cooldown.tick(time.delta());

        if extractor.cooldown.finished() && extractor_inventory.remaining_space() > 0 {
            let mut astre_inventory = q_astre_inventories.get_mut(parent.get()).unwrap();

            let mut rng = rand::thread_rng();
            if let Some(random_item_ids) = &extractor.cached_item_ids {
                let random_item_id =
                    random_item_ids.choose_weighted(&mut rng, |id| astre_inventory.quantity(id));

                if let Ok(item_id) = random_item_id {
                    let quantity = astre_inventory
                        .quantity(&item_id)
                        .min(extractor.amount_per_tick);

                    astre_inventory.transfer_to(
                        &mut extractor_inventory,
                        item_id.clone(),
                        quantity,
                    );
                } else {
                    extractor.cached_item_ids = None;
                }
            } else {
                extractor.cached_item_ids = Some(
                    astre_inventory
                        .all_ids()
                        .iter()
                        .filter(|id| {
                            ELEMENTS
                                .get(*id)
                                .map_or(false, |element| element.state == extractor.element_state)
                        })
                        .map(|id| (*id).clone())
                        .collect::<Vec<_>>(),
                );
            }
        }
    }
}
