use bevy::prelude::*;

use crate::{buildings::BuildingId, enum_map, items::ItemId};

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

    #[inline]
    pub fn time(&self) -> f32 {
        self.time
    }

    #[inline]
    pub fn inputs(&self) -> RecipeItemQuantities {
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

enum_map! {
    RecipeId => Recipe {
        SmeltElectroniteOre = Recipe::new_items(
            &[(ItemId::ElectroniteOre, 1)],
            &[(ItemId::Electronite, 1)],
            1.,
        ),

        CraftPlasmaFuel = Recipe::new_items(
            &[(ItemId::Photonite, 1), (ItemId::Gravitonite, 1)],
            &[(ItemId::PlasmaFuel, 1)],
            1.,
        ),

        CraftComputingCore = Recipe::new_items(
            &[(ItemId::Electronite, 1), (ItemId::QuarkCrystal, 1)],
            &[(ItemId::ComputingCore, 1)],
            2.,
        ),

        Foundry = Recipe::new_building(
            &[(ItemId::Astrium, 10)],
            BuildingId::Foundry,
            3.,
        ),

        SpawnCargoShuttle = Recipe::new_building(
            &[(ItemId::Astrium, 10), (ItemId::ComputingCore, 3), (ItemId::PlasmaFuel, 5)],
            BuildingId::CargoShuttle,
            3.,
        ),
    }
}
