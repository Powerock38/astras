use bevy::{
    ecs::{entity::MapEntities, reflect::ReflectMapEntities},
    prelude::*,
};
use uuid::Uuid;

use crate::{
    data::ItemId,
    items::{Inventory, ItemMap},
};

#[derive(PartialEq, Eq, Clone, Copy, Reflect, Default, Debug)]
pub enum LogisticScope {
    #[default]
    Planet,
    SolarSystem,
}

impl LogisticScope {
    pub fn opposite(self) -> Self {
        match self {
            LogisticScope::Planet => LogisticScope::SolarSystem,
            LogisticScope::SolarSystem => LogisticScope::Planet,
        }
    }
}

impl std::fmt::Display for LogisticScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogisticScope::Planet => write!(f, "Planet"),
            LogisticScope::SolarSystem => write!(f, "Solar System"),
        }
    }
}

#[derive(Component, MapEntities, Reflect, Default, Debug)]
#[reflect(Component, Default, MapEntities)]
pub struct LogisticRequest {
    id: Uuid,
    items: ItemMap,
    scope: LogisticScope,
    #[entities]
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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn scope(&self) -> &LogisticScope {
        &self.scope
    }

    pub fn items(&self) -> &ItemMap {
        &self.items
    }

    pub fn set_items(&mut self, items: ItemMap) {
        self.id = Uuid::new_v4();
        self.items = items;
    }

    pub fn add_item(&mut self, id: ItemId, quantity: u32) {
        if let Some(item) = self.items.get_mut(&id) {
            *item += quantity;
        } else {
            self.items.insert(id, quantity);
        }
    }

    pub fn remove_item(&mut self, id: ItemId, quantity: u32) {
        if let Some(item) = self.items.get_mut(&id) {
            *item -= quantity;
            if *item == 0 {
                self.items.remove(&id);
            }
        }
    }

    pub fn compute_fulfillment_percentage(&self, provider_inventory: &Inventory) -> u32 {
        self.items.iter().fold(0, |total, (id, quantity)| {
            total + provider_inventory.quantity(*id).min(*quantity)
        }) * 100
            / (self
                .items
                .iter()
                .fold(0, |total, (_, quantity)| total + quantity)
                * 100)
    }
}

// If a building has a LogisticProvider component, its Inventory component will be used to fulfill LogisticRequests
#[derive(Component, MapEntities, Reflect, Default, Debug)]
#[reflect(Component, Default, MapEntities)]
pub struct LogisticProvider {
    scope: LogisticScope,
    #[entities]
    pub freights: Vec<Entity>,
}

impl LogisticProvider {
    pub fn new(scope: LogisticScope) -> Self {
        Self {
            scope,
            freights: Vec::new(),
        }
    }

    pub fn scope(&self) -> &LogisticScope {
        &self.scope
    }
}

#[derive(Reflect, MapEntities, Debug, Clone, Copy)]
#[reflect(MapEntities)]
pub struct LogisticJourney {
    request_id: Uuid,
    #[entities]
    provider: Entity,
    #[entities]
    requester: Entity,
}

impl LogisticJourney {
    pub fn new(request_id: Uuid, provider: Entity, requester: Entity) -> Self {
        assert_ne!(provider, requester);

        Self {
            request_id,
            provider,
            requester,
        }
    }

    pub fn request_id(&self) -> Uuid {
        self.request_id
    }

    pub fn provider(&self) -> Entity {
        self.provider
    }

    pub fn requester(&self) -> Entity {
        self.requester
    }
}
