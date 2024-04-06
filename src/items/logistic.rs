use bevy::{prelude::*, utils::Uuid};

use super::{Inventory, ItemMap};

#[derive(Component, Debug)]
pub struct LogisticRequest {
    id: Uuid,
    items: ItemMap,
    pub freighters: Vec<Entity>,
}

impl LogisticRequest {
    pub fn new(items: ItemMap) -> Self {
        Self {
            id: Uuid::new_v4(),
            items,
            freighters: Vec::new(),
        }
    }

    #[inline]
    pub fn items(&self) -> &ItemMap {
        &self.items
    }

    #[inline]
    pub fn set_items(&mut self, items: ItemMap) {
        self.id = Uuid::new_v4();
        self.items = items;
    }

    #[inline]
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn compute_fulfillment_percentage(&self, inventory: &Inventory) -> u32 {
        self.items.iter().fold(0, |total, (id, quantity)| {
            total + inventory.quantity(id).min(*quantity)
        }) * 100
            / (self
                .items
                .iter()
                .fold(0, |total, (_, quantity)| total + quantity)
                * 100)
    }
}

// If a building has a LogisticProvider component, its Inventory component will be used to fulfill LogisticRequests
#[derive(Component, Default, Debug)]
pub struct LogisticProvider {
    pub freighters: Vec<Entity>,
}

#[derive(Debug)]
pub struct LogisticJourney {
    request_id: Uuid,
    provider: Entity,
    requester: Entity,
}

impl LogisticJourney {
    pub fn new(request_id: Uuid, provider: Entity, requester: Entity) -> Self {
        Self {
            request_id,
            provider,
            requester,
        }
    }

    #[inline]
    pub fn request_id(&self) -> Uuid {
        self.request_id
    }

    #[inline]
    pub fn provider(&self) -> Entity {
        self.provider
    }

    #[inline]
    pub fn requester(&self) -> Entity {
        self.requester
    }
}
