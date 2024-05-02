use bevy::prelude::*;
use rand::{seq::IteratorRandom, Rng};

use crate::universe::{PlanetColors, NB_COLORS};

pub struct Element {
    pub color: Color,
    pub state: ElementState,
}

impl Element {
    pub const fn new(color: Color, state: ElementState) -> Self {
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
    pub id: &'static str,
    pub quantity: u32,
}

impl ElementOnAstre {
    pub fn random_elements(
        n: u32,
        max_quantity: u32,
        states: &[ElementState],
    ) -> Vec<ElementOnAstre> {
        let mut rng = rand::thread_rng();

        ELEMENTS
            .entries()
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
                let quantity = rng.gen_range(1..=max_quantity);

                ElementOnAstre {
                    id: element,
                    quantity,
                }
            })
            .collect()
    }

    pub fn get_color(elements: &[ElementOnAstre]) -> Color {
        let total_mass: u32 = elements.iter().map(|e| e.quantity).sum();

        elements
            .iter()
            .map(|e| {
                let element = &ELEMENTS[e.id];
                let ratio = e.quantity as f32 / total_mass as f32;
                element.color * ratio
            })
            .fold(Color::BLACK, |acc, c| acc + c)
    }

    pub fn get_colors(elements: &[ElementOnAstre]) -> PlanetColors {
        let mut elements = elements.to_vec();
        elements.sort_by_key(|e| e.quantity);

        let mut color = elements.first().map(|e| ELEMENTS[e.id].color).unwrap();
        let colors = &mut [color; NB_COLORS];

        for (i, color_item) in colors.iter_mut().enumerate().skip(1) {
            color = elements.get(i).map_or(color, |e| ELEMENTS[e.id].color);
            *color_item = color;
        }

        *colors
    }
}

pub static ELEMENTS: phf::Map<&'static str, Element> = phf::phf_map! {
    // Atmosphere
    "aer" => Element::new(Color::ANTIQUE_WHITE, ElementState::Gas),

    // Oceans
    "aqua" => Element::new(Color::BLUE, ElementState::Liquid),

    // Rocks
    "terra" => Element::new(Color::MAROON, ElementState::Solid),
    "astrium" => Element::new(Color::SILVER, ElementState::Solid),
    "electronite_ore" => Element::new(Color::ORANGE_RED, ElementState::Solid),
    "quark_crystal" => Element::new(Color::FUCHSIA, ElementState::Solid),

    // Stars
    "photonite" => Element::new(Color::YELLOW, ElementState::Plasma),
    "neutronite" => Element::new(Color::AQUAMARINE, ElementState::Plasma),
    "gravitonite" => Element::new(Color::RED, ElementState::Plasma),
};
