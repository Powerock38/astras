use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Astre {
    pub temperature: u32, // in Kelvin  // TODO: why
    pub surface_radius: f32,
    pub atmosphere_radius: f32,
}
