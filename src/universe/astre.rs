use bevy::prelude::*;

use crate::{items::Inventory, universe::update_ship_mining};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Inventory)]
pub struct Astre {
    surface_radius: f32,
    atmosphere_radius: f32,
}

impl Astre {
    pub fn new(surface_radius: f32, atmosphere_radius: f32) -> Self {
        Self {
            surface_radius,
            atmosphere_radius,
        }
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
    for entity in &query {
        commands.entity(entity).observe(update_ship_mining);
    }
}
