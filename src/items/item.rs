use bevy::utils::HashMap;
use phf::phf_map;

pub struct Item {
    pub name: &'static str,
    pub description: &'static str,
}

pub type ItemMap = HashMap<String, u32>; // Item ID -> Quantity

pub static ITEMS: phf::Map<&'static str, Item> = phf_map! {
    // Elements
    "electronite_ore" => Item {
        name: "Electronite Ore",
        description: "Shiny ore that can be refined",
    },
    "quark_crystal" => Item {
        name: "Quark Crystal",
        description: "Vibrating crystal used for computing",
    },
    "astrium" => Item {
        name: "Astrium",
        description: "Solid but light material used for megastructure construction",
    },
    "photonite" => Item {
        name: "Photonite",
        description: "Light-emitting high-energy element",
    },
    "gravitonite" => Item {
        name: "Gravitonite",
        description: "Volatile element used for propulsion",
    },
    "neutronite" => Item {
        name: "Neutronite",
        description: "Dense element used for shielding",
    },
    "aer" => Item {
        name: "Aer",
        description: "Mundane gas",
    },
    "aqua" => Item {
        name: "Aqua",
        description: "Mundane liquid",
    },
    "terra" => Item {
        name: "Terra",
        description: "Mundane solid",
    },

    // Basic processed materials
    "electronite" => Item {
        name: "Electronite",
        description: "Highly conductive material",
    },
    "computing_core" => Item {
        name: "Computing Core",
        description: "Calculates very fast",
    },
    "plasma_fuel" => Item {
        name: "Plasma fuel",
        description: "High-energy spaceship fuel",
    },
};
