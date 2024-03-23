use bevy::prelude::*;
use bevy::render::mesh::CircleMeshBuilder;
use std::f32::consts::PI;

#[derive(Component, Debug)]
pub struct Astre {
    pub temperature: u32, // in Kelvin  // TODO: why
    pub surface_radius: f32,
    pub atmosphere_radius: f32,
}

pub fn circle_mesh(radius: f32) -> Mesh {
    const ERR: f32 = 10.0;
    let vertices = (PI / (1. - ERR / radius).acos()).ceil() as usize;
    CircleMeshBuilder::new(radius, vertices).build()
}
