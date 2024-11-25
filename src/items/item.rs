use bevy::{prelude::*, utils::HashMap};

use crate::enum_map;

pub struct Item {
    pub name: &'static str,
    pub description: &'static str,
}

pub type ItemMap = HashMap<ItemId, u32>; // Item ID -> Quantity

enum_map! {
    ItemId => Item {
        // Elements
        ElectroniteOre = Item {
            name: "Electronite Ore",
            description: "Shiny ore that can be refined",
        },
        QuarkCrystal = Item {
            name: "Quark Crystal",
            description: "Vibrating crystal used for computing",
        },
        Astrium = Item {
            name: "Astrium",
            description: "Solid but light material used for megastructure construction",
        },
        Photonite = Item {
            name: "Photonite",
            description: "Light-emitting high-energy element",
        },
        Gravitonite = Item {
            name: "Gravitonite",
            description: "Volatile element used for propulsion",
        },
        Neutronite = Item {
            name: "Neutronite",
            description: "Dense element used for shielding",
        },
        Aer = Item {
            name: "Aer",
            description: "Mundane gas",
        },
        Aqua = Item {
            name: "Aqua",
            description: "Mundane liquid",
        },
        Terra = Item {
            name: "Terra",
            description: "Mundane solid",
        },

        // Basic processed materials
        Electronite = Item {
            name: "Electronite",
            description: "Highly conductive material",
        },
        ComputingCore = Item {
            name: "Computing Core",
            description: "Calculates very fast",
        },
        PlasmaFuel = Item {
            name: "Plasma fuel",
            description: "High-energy spaceship fuel",
        },
    }
}
