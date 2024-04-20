use bevy::prelude::*;
use bevy::render::mesh::CircleMeshBuilder;
use bevy::utils::Uuid;
use std::f32::consts::PI;

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
