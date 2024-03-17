use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Element {
    pub name: &'static str,
    pub color: Color,
}
