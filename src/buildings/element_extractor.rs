use bevy::prelude::*;
use rand::prelude::SliceRandom;

use crate::{
    astre::Astre,
    items::{ElementState, Inventory, ELEMENTS},
};

#[derive(Component)]
pub struct ElementExtractor {
    inventory: Inventory,
    cooldown: Timer,
    amount_per_tick: u32,
    element_state: ElementState,
}

impl ElementExtractor {
    pub fn new_solid() -> Self {
        Self {
            element_state: ElementState::Solid,
            inventory: Inventory::new(),
            cooldown: Timer::from_seconds(1., TimerMode::Repeating),
            amount_per_tick: 10,
        }
    }
}

pub fn update_element_extractors(
    time: Res<Time>,
    mut q_extractor: Query<(&mut ElementExtractor, &Parent)>,
    mut q_astre_inventories: Query<&mut Inventory, With<Astre>>,
) {
    for (mut extractor, parent) in q_extractor.iter_mut() {
        extractor.cooldown.tick(time.delta());
        if extractor.cooldown.finished() {
            let mut astre_inventory = q_astre_inventories.get_mut(parent.get()).unwrap();

            let mut rng = rand::thread_rng();
            let random_item = astre_inventory
                .all_ids()
                .iter()
                .filter(|id| {
                    ELEMENTS
                        .get(*id)
                        .map_or(false, |element| element.state == extractor.element_state)
                })
                .collect::<Vec<_>>() // TODO: optimize by caching the list of ids
                .choose_weighted(&mut rng, |id| astre_inventory.quantity(**id))
                .ok()
                .map(|id| **id);

            if let Some(item) = random_item {
                let quantity = astre_inventory
                    .quantity(item)
                    .min(extractor.amount_per_tick);

                astre_inventory.transfer_to(&mut extractor.inventory, item, quantity);

                println!("Mined {} {}", quantity, item);
            }
        }
    }
}
