use bevy::prelude::*;
use rand::prelude::SliceRandom;

use crate::{
    universe::Astre,
    items::{ElementState, Inventory, LogisticProvider, LogisticScope, ELEMENTS},
};

#[derive(Bundle)]
pub struct ElementExtractorBundle {
    pub element_extractor: ElementExtractor,
    pub inventory: Inventory,
    pub logistic_provider: LogisticProvider,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ElementExtractor {
    cooldown: Timer,
    amount_per_tick: u32,
    element_state: ElementState,
}

impl ElementExtractorBundle {
    pub fn new_solid() -> Self {
        Self {
            element_extractor: ElementExtractor {
                element_state: ElementState::Solid,
                cooldown: Timer::from_seconds(1., TimerMode::Repeating),
                amount_per_tick: 100,
            },
            inventory: Inventory::new(1000),
            logistic_provider: LogisticProvider::new(LogisticScope::Planet),
        }
    }

    pub fn new_liquid() -> Self {
        Self {
            element_extractor: ElementExtractor {
                element_state: ElementState::Liquid,
                cooldown: Timer::from_seconds(1., TimerMode::Repeating),
                amount_per_tick: 1000,
            },
            inventory: Inventory::new(5000),
            logistic_provider: LogisticProvider::new(LogisticScope::Planet),
        }
    }

    pub fn new_gas() -> Self {
        Self {
            element_extractor: ElementExtractor {
                element_state: ElementState::Gas,
                cooldown: Timer::from_seconds(1., TimerMode::Repeating),
                amount_per_tick: 500,
            },
            inventory: Inventory::new(10_000),
            logistic_provider: LogisticProvider::new(LogisticScope::Planet),
        }
    }

    pub fn new_plasma() -> Self {
        Self {
            element_extractor: ElementExtractor {
                element_state: ElementState::Plasma,
                cooldown: Timer::from_seconds(1., TimerMode::Repeating),
                amount_per_tick: 10,
            },
            inventory: Inventory::new(100),
            logistic_provider: LogisticProvider::new(LogisticScope::Planet),
        }
    }
}

pub fn update_element_extractors(
    time: Res<Time>,
    mut q_extractor: Query<(&mut ElementExtractor, &mut Inventory, &Parent), Without<Astre>>,
    mut q_astre_inventories: Query<&mut Inventory, With<Astre>>,
) {
    for (mut extractor, mut extractor_inventory, parent) in q_extractor.iter_mut() {
        extractor.cooldown.tick(time.delta());

        if extractor.cooldown.finished() && extractor_inventory.remaining_space() > 0 {
            let mut astre_inventory = q_astre_inventories.get_mut(parent.get()).unwrap();

            let mut rng = rand::thread_rng();
            let random_item_ids = astre_inventory
                .all_ids()
                .iter()
                .filter(|id| {
                    ELEMENTS
                        .get(*id)
                        .map_or(false, |element| element.state == extractor.element_state)
                })
                .map(|id| (*id).clone())
                .collect::<Vec<_>>(); // TODO: optimize by caching the list of ids

            let random_item_id = random_item_ids
                .choose_weighted(&mut rng, |id| astre_inventory.quantity(id))
                .ok()
                .cloned();

            if let Some(item_id) = random_item_id {
                let quantity = astre_inventory
                    .quantity(&item_id)
                    .min(extractor.amount_per_tick);

                let q = astre_inventory.transfer_to(
                    &mut extractor_inventory,
                    item_id.clone(),
                    quantity,
                );

                println!(
                    "Mined {} {} - from {} to {}",
                    q,
                    item_id,
                    astre_inventory.remaining_space(),
                    extractor_inventory.remaining_space()
                );
            }
        }
    }
}
