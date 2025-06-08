use crate::data::{BuildingId, ItemId};

type RecipeItemQuantities = &'static [(ItemId, u32)];

#[derive(Clone, Copy, Debug)]
pub enum RecipeOutputs {
    Items(RecipeItemQuantities),
    Building(BuildingId),
}

#[derive(Clone, Copy, Debug)]
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

    pub const fn new_building(inputs: RecipeItemQuantities, output: BuildingId, time: f32) -> Self {
        Self {
            inputs,
            outputs: RecipeOutputs::Building(output),
            time,
        }
    }


    pub fn time(&self) -> f32 {
        self.time
    }


    pub fn inputs(&self) -> RecipeItemQuantities {
        self.inputs
    }


    pub fn outputs(&self) -> RecipeOutputs {
        self.outputs
    }


    pub fn inputs_quantity(&self) -> u32 {
        self.inputs.iter().map(|(_, quantity)| quantity).sum()
    }


    pub fn outputs_quantity(&self) -> u32 {
        match self.outputs {
            RecipeOutputs::Items(items) => items.iter().map(|(_, quantity)| quantity).sum(),
            RecipeOutputs::Building(_) => 0,
        }
    }
}
