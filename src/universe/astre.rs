use bevy::prelude::*;

use crate::{items::Inventory, universe::update_ship_mining};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Inventory)]
pub struct Astre {
    surface: f32,
    atmosphere: f32,
    close_orbit: f32,
}

impl Astre {
    pub fn new(surface: f32, atmosphere: f32, close_orbit: f32) -> Self {
        Self {
            surface,
            atmosphere,
            close_orbit,
        }
    }

    #[inline]
    pub fn surface_radius(&self) -> f32 {
        self.surface
    }

    #[inline]
    pub fn atmosphere_radius(&self) -> f32 {
        self.surface + self.atmosphere
    }

    #[inline]
    pub fn has_atmosphere(&self) -> bool {
        self.atmosphere > 0.0
    }

    #[inline]
    pub fn close_orbit_radius(&self) -> f32 {
        self.surface + self.atmosphere + self.close_orbit
    }
}

pub fn scan_astres(mut commands: Commands, query: Query<Entity, Added<Astre>>) {
    for entity in &query {
        commands.entity(entity).observe(update_ship_mining);
    }
}
