type RecipeItemQuantities = &'static [(&'static str, u32)];

#[derive(Clone, Copy)]
pub enum RecipeOutputs {
    Items(RecipeItemQuantities),
    Building(&'static str),
}

#[derive(Clone, Copy)]
pub struct Recipe {
    inputs: RecipeItemQuantities,
    outputs: RecipeOutputs,
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
            outputs: RecipeOutputs::Items(output),
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
            outputs: RecipeOutputs::Building(output),
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
    pub fn outputs(&self) -> RecipeOutputs {
        self.outputs
    }

    #[inline]
    pub fn inputs_quantity(&self) -> u32 {
        self.inputs.iter().map(|(_, quantity)| quantity).sum()
    }

    #[inline]
    pub fn outputs_quantity(&self) -> u32 {
        match self.outputs {
            RecipeOutputs::Items(items) => items.iter().map(|(_, quantity)| quantity).sum(),
            RecipeOutputs::Building(_) => 0,
        }
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
    "foundry" => Recipe::new_building(
        &[("astrium", 10)],
        "foundry",
        3.,
    ),
    "spawn_cargo_shuttle" => Recipe::new_building(
        &[("astrium", 10), ("computing_core", 3), ("plasma_fuel", 5)],
        "cargo_shuttle",
        3.,
    ),
};
