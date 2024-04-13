use bevy::prelude::*;
use rand::{seq::IteratorRandom, Rng};

use crate::astres::NB_COLORS;

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

    pub fn get_colors(elements: &[ElementOnAstre]) -> [Color; NB_COLORS] {
        let mut elements = elements.to_vec();
        elements.sort_by_key(|e| e.quantity);

        let mut color = elements.get(0).map(|e| ELEMENTS[e.id].color).unwrap();
        let mut colors = vec![color; NB_COLORS];
        for i in 1..NB_COLORS {
            color = elements
                .get(i)
                .map(|e| ELEMENTS[e.id].color)
                .unwrap_or(color);
            colors[i] = color;
        }

        colors.try_into().unwrap()
    }
}

pub static ELEMENTS: phf::Map<&'static str, Element> = phf::phf_map! {
    "aer" => Element::new(Color::rgba(1.0, 1.0, 1.0, 0.5), ElementState::Gas),
    "aqua" => Element::new(Color::BLUE, ElementState::Liquid),
    //"terra" => Element::new(Color::MAROON, ElementState::Solid),
    //"rock" => Element::new(Color::GRAY, ElementState::Solid),
    "electronite_ore" => Element::new(Color::ORANGE_RED, ElementState::Solid),
    //"quark_crystal" => Element::new(Color::TURQUOISE, ElementState::Solid),
    "photonite" => Element::new(Color::YELLOW, ElementState::Plasma),
    "neutronite" => Element::new(Color::ALICE_BLUE, ElementState::Plasma),
    "gravitonite" => Element::new(Color::RED, ElementState::Plasma),
};
