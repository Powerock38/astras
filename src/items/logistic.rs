use bevy::prelude::*;

use super::{Inventory, ItemMap};

#[derive(Component, Debug)]
pub struct LogisticRequest {
    items: ItemMap,
}

impl LogisticRequest {
    pub fn new(items: ItemMap) -> Self {
        Self { items }
    }

    #[inline]
    pub fn items(&self) -> &ItemMap {
        &self.items
    }

    pub fn can_be_partially_fullfilled_by(&self, inventory: &Inventory) -> bool {
        self.items.keys().any(|id| inventory.quantity(id) != 0)
    }
}

// If a building has a LogisticProvider component, its Inventory component will be used to fulfill LogisticRequests
#[derive(Component)]
pub struct LogisticProvider;

#[derive(Debug)]
pub struct LogisticJourney {
    provider: Entity,
    requester: Entity,
}

impl LogisticJourney {
    pub fn new(provider: Entity, requester: Entity) -> Self {
        Self {
            provider,
            requester,
        }
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
