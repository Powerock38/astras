use bevy::prelude::*;
use rand::prelude::SliceRandom;

use crate::{
    data::{ItemId, ELEMENTS},
    items::{ElementState, Inventory, LogisticProvider, LogisticScope},
    universe::Astre,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Inventory, LogisticProvider(|| LogisticProvider::new(LogisticScope::Planet)))]
pub struct Extractor {
    cooldown: Timer,
    amount_per_tick: u32,
    element_state: ElementState,
    cached_item_ids: Option<Vec<ItemId>>,
}

impl Extractor {
    pub fn new_solid() -> Self {
        Self {
            element_state: ElementState::Solid,
            cooldown: Timer::from_seconds(1., TimerMode::Repeating),
            amount_per_tick: 100,
            cached_item_ids: None,
        }
    }

    pub fn new_liquid() -> Self {
        Self {
            element_state: ElementState::Liquid,
            cooldown: Timer::from_seconds(1., TimerMode::Repeating),
            amount_per_tick: 1000,
            cached_item_ids: None,
        }
    }

    pub fn new_gas() -> Self {
        Self {
            element_state: ElementState::Gas,
            cooldown: Timer::from_seconds(1., TimerMode::Repeating),
            amount_per_tick: 500,
            cached_item_ids: None,
        }
    }

    pub fn new_plasma() -> Self {
        Self {
            element_state: ElementState::Plasma,
            cooldown: Timer::from_seconds(1., TimerMode::Repeating),
            amount_per_tick: 10,
            cached_item_ids: None,
        }
    }
}

pub fn update_extractors(
    time: Res<Time>,
    mut q_extractors: Query<(&mut Extractor, &mut Inventory, &Parent), Without<Astre>>,
    mut q_astre_inventories: Query<&mut Inventory, With<Astre>>,
) {
    for (mut extractor, mut extractor_inventory, parent) in &mut q_extractors {
        extractor.cooldown.tick(time.delta());

        if extractor.cooldown.finished() && extractor_inventory.remaining_space() > 0 {
            let mut astre_inventory = q_astre_inventories.get_mut(parent.get()).unwrap();

            let mut rng = rand::thread_rng();
            if let Some(random_item_ids) = &extractor.cached_item_ids {
                let random_item_id =
                    random_item_ids.choose_weighted(&mut rng, |id| astre_inventory.quantity(*id));

                if let Ok(item_id) = random_item_id {
                    let quantity = astre_inventory
                        .quantity(*item_id)
                        .min(extractor.amount_per_tick);

                    astre_inventory.transfer_to(&mut extractor_inventory, *item_id, quantity);
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
                                .is_some_and(|element| element.state == extractor.element_state)
                        })
                        .copied()
                        .collect::<Vec<_>>(),
                );
            }
        }
    }
}
