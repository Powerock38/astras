use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    data::{ItemId, ELEMENTS},
    universe::{PlanetColors, NB_COLORS},
};

pub struct Element {
    pub color: Srgba,
    pub state: ElementState,
}

impl Element {
    pub const fn new(color: Srgba, state: ElementState) -> Self {
        Self { color, state }
    }
}

#[derive(PartialEq, Reflect, Default)]
pub enum ElementState {
    #[default]
    Solid,
    Liquid,
    Gas,
    Plasma,
}

#[derive(Clone, Copy)]
pub struct ElementOnAstre {
    pub id: ItemId,
    pub quantity: u32,
}

impl ElementOnAstre {
    pub fn random_elements(
        mut rng: &mut StdRng,
        n: u32,
        max_quantity: u32,
        states: &[ElementState],
    ) -> Vec<ElementOnAstre> {
        ELEMENTS
            .iter()
            .filter_map(|(id, element)| {
                if states.contains(&element.state) {
                    Some(*id)
                } else {
                    None
                }
            })
            .choose_multiple(&mut rng, n as usize)
            .iter()
            .map(|&element| {
                let quantity = rng.random_range(1..=max_quantity);

                ElementOnAstre {
                    id: element,
                    quantity,
                }
            })
            .collect()
    }

    pub fn get_color(elements: &[ElementOnAstre]) -> LinearRgba {
        let total_mass: u32 = elements.iter().map(|e| e.quantity).sum();

        elements
            .iter()
            .map(|e| {
                let element = &ELEMENTS[&e.id];
                let ratio = e.quantity as f32 / total_mass as f32;
                element.color * ratio
            })
            .fold(Color::BLACK.into(), |acc, c| acc + c.into())
    }

    pub fn get_colors(elements: &[ElementOnAstre]) -> PlanetColors {
        if elements.is_empty() {
            return [Color::BLACK.into(); NB_COLORS];
        }

        let mut elements = elements.to_vec();
        elements.sort_by_key(|e| e.quantity);

        let mut color = elements.first().map(|e| ELEMENTS[&e.id].color).unwrap();
        let colors = &mut [color.into(); NB_COLORS];

        for (i, color_item) in colors.iter_mut().enumerate().skip(1) {
            color = elements.get(i).map_or(color, |e| ELEMENTS[&e.id].color);
            *color_item = color.into();
        }

        *colors
    }
}
