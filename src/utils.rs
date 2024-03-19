use bevy::render::mesh::{CircleMeshBuilder, Mesh};
use std::f32::consts::PI;

pub fn circle_mesh(radius: f32) -> Mesh {
    const ERR: f32 = 10.0;
    let vertices = (PI / (1. - ERR / radius).acos()).ceil() as usize;
    CircleMeshBuilder::new(radius, vertices).build()
}
