use crate::items::ITEMS;

use super::Inventory;

#[derive(Clone, Copy)]
pub struct Recipe {
    inputs: &'static [(&'static str, u32)],
    outputs: &'static [(&'static str, u32)],
    time: f32,
}

impl Recipe {
    pub fn can_craft(&self, inventory: &Inventory) -> bool {
        self.inputs
            .iter()
            .all(|(id, quantity)| inventory.quantity(id) >= *quantity)
    }

    #[inline]
    pub fn time(&self) -> f32 {
        self.time
    }

    fn slice_to_string(slice: &[(&'static str, u32)]) -> String {
        slice
            .iter()
            .map(|(id, quantity)| format!("{} (x{})", ITEMS[id].name, quantity))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn text(&self) -> String {
        format!(
            "{} -> {}",
            Self::slice_to_string(self.inputs),
            Self::slice_to_string(self.outputs)
        )
    }
}

pub static RECIPES: phf::Map<&'static str, Recipe> = phf::phf_map! {
    "smelt_electronite_ore" => Recipe {
        inputs: &[("electronite_ore", 1)],
        outputs: &[("electronite", 1)],
        time: 1.,
    },
};
