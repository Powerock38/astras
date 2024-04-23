use crate::items::ITEMS;

type RecipeItemQuantities = &'static [(&'static str, u32)];

#[derive(Clone, Copy)]
pub enum RecipeOutput {
    Items(RecipeItemQuantities),
    Building(&'static str),
}

#[derive(Clone, Copy)]
pub struct Recipe {
    inputs: RecipeItemQuantities,
    output: RecipeOutput,
    time: f32,
}

impl Recipe {
    pub const fn new_items(
        inputs: RecipeItemQuantities,
        output: RecipeItemQuantities,
        time: f32,
    ) -> Self {
        Self {
            inputs,
            output: RecipeOutput::Items(output),
            time,
        }
    }

    pub const fn new_building(
        inputs: RecipeItemQuantities,
        output: &'static str,
        time: f32,
    ) -> Self {
        Self {
            inputs,
            output: RecipeOutput::Building(output),
            time,
        }
    }

    #[inline]
    pub fn time(&self) -> f32 {
        self.time
    }

    #[inline]
    pub fn inputs(&self) -> &'static [(&'static str, u32)] {
        self.inputs
    }

    #[inline]
    pub fn output(&self) -> RecipeOutput {
        self.output
    }

    #[inline]
    pub fn inputs_space_needed(&self) -> u32 {
        self.inputs.iter().map(|(_, quantity)| quantity).sum()
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
            match self.output {
                RecipeOutput::Items(slice) => Self::slice_to_string(slice),
                RecipeOutput::Building(name) => name.to_string(),
            }
        )
    }
}

pub static RECIPES: phf::Map<&'static str, Recipe> = phf::phf_map! {
    "smelt_electronite_ore" => Recipe::new_items(
        &[("electronite_ore", 1)],
        &[("electronite", 1)],
         1.,
    ),
    "craft_plasma_fuel" => Recipe::new_items(
        &[("photonite", 1), ("gravitonite", 1)],
        &[("plasma_fuel", 1)],
        1.,
    ),
    "craft_computing_core" => Recipe::new_items(
        &[("electronite", 1), ("quark_crystal", 1)],
        &[("computing_core", 1)],
         2.,
    ),

    // Buildings
    "spawn_cargo_shuttle" => Recipe::new_building(
        &[("astrium", 10), ("computing_core", 3), ("plasma_fuel", 5)],
        "cargo_shuttle",
        3.,
    ),
};
