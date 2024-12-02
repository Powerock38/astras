use bevy::prelude::*;
use uuid::Uuid;

use crate::{items::Inventory, universe::update_ship_mining};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Inventory)]
pub struct Astre {
    uuid: Uuid,
    surface_radius: f32,
    atmosphere_radius: f32,
}

impl Astre {
    pub fn new(surface_radius: f32, atmosphere_radius: f32) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            surface_radius,
            atmosphere_radius,
        }
    }

    #[inline]
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    #[inline]
    pub fn surface_radius(&self) -> f32 {
        self.surface_radius
    }

    #[inline]
    pub fn atmosphere_radius(&self) -> f32 {
        self.atmosphere_radius
    }
}

pub fn scan_astres(mut commands: Commands, query: Query<Entity, Added<Astre>>) {
    for entity in query.iter() {
        commands.entity(entity).observe(update_ship_mining);
    }
}
