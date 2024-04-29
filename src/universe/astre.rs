use bevy::{prelude::*, render::mesh::CircleMeshBuilder, utils::Uuid};
use bevy_mod_picking::prelude::*;
use std::f32::consts::PI;

use crate::items::{ElementOnAstre, Inventory};

use super::update_ship_mining;

#[derive(Bundle)]
pub struct AstreBundle {
    astre: Astre,
    inventory: Inventory,
}

impl AstreBundle {
    pub fn new(
        surface_radius: f32,
        atmosphere_radius: f32,
        composition: Vec<ElementOnAstre>,
    ) -> Self {
        Self {
            astre: Astre::new(surface_radius, atmosphere_radius),
            inventory: Inventory::from(composition),
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
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

pub fn circle_mesh(radius: f32) -> Mesh {
    const ERR: f32 = 10.0;
    let vertices = (PI / (1. - ERR / radius).acos()).ceil() as usize;
    CircleMeshBuilder::new(radius, vertices).build()
}

pub fn scan_astres(mut commands: Commands, query: Query<Entity, Added<Astre>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(On::<Pointer<Down>>::run(update_ship_mining));
    }
}
