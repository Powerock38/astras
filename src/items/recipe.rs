use crate::items::ITEMS;

type RecipeItemQuantities = &'static [(&'static str, u32)];

#[derive(Clone, Copy)]
pub struct Recipe {
    inputs: RecipeItemQuantities,
    outputs: RecipeItemQuantities,
    time: f32,
}

impl Recipe {
    #[inline]
    pub fn time(&self) -> f32 {
        self.time
    }

    #[inline]
    pub fn inputs(&self) -> &'static [(&'static str, u32)] {
        self.inputs
    }

    #[inline]
    pub fn outputs(&self) -> &'static [(&'static str, u32)] {
        self.outputs
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
