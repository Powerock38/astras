use bevy::prelude::*;
use uuid::Uuid;

use crate::items::{Inventory, ItemMap};

#[derive(PartialEq, Eq, Reflect, Default, Debug)]
pub enum LogisticScope {
    #[default]
    Planet,
    SolarSystem,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct LogisticRequest {
    id: Uuid,
    items: ItemMap,
    scope: LogisticScope,
    pub freights: Vec<Entity>,
}

impl LogisticRequest {
    pub fn new(items: ItemMap, scope: LogisticScope) -> Self {
        Self {
            id: Uuid::new_v4(),
            items,
            scope,
            freights: Vec::new(),
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

    #[inline]
    pub fn scope(&self) -> &LogisticScope {
        &self.scope
    }

    pub fn compute_fulfillment_percentage(&self, provider_inventory: &Inventory) -> u32 {
        self.items.iter().fold(0, |total, (id, quantity)| {
            total + provider_inventory.quantity(id).min(*quantity)
        }) * 100
            / (self
                .items
                .iter()
                .fold(0, |total, (_, quantity)| total + quantity)
                * 100)
    }
}

// If a building has a LogisticProvider component, its Inventory component will be used to fulfill LogisticRequests
#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct LogisticProvider {
    scope: LogisticScope,
    pub freights: Vec<Entity>,
}

impl LogisticProvider {
    pub fn new(scope: LogisticScope) -> Self {
        Self {
            scope,
            freights: Vec::new(),
        }
    }

    #[inline]
    pub fn scope(&self) -> &LogisticScope {
        &self.scope
    }
}

#[derive(Reflect, Debug, Clone, Copy)]
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
