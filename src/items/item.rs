use phf::phf_map;

pub struct Item {
    pub name: &'static str,
    pub description: &'static str,
}

pub static ITEMS: phf::Map<&'static str, Item> = phf_map! {
    "electronite_ore" => Item {
        name: "Electronite Ore",
        description: "A shiny ore that can be refined into electronite",
    },
    "electronite" => Item {
        name: "Electronite",
        description: "The base element for electronics",
    },
    "quark_crystal" => Item {
        name: "Quark Crystal",
        description: "The base element for computing",
    },
    "photonite" => Item {
        name: "Photonite",
        description: "The base element for energy",
    },
    "gravitonite" => Item {
        name: "Gravitonite",
        description: "The base element for interstellar travel",
    },
    "aer" => Item {
        name: "Aer",
        description: "Mundane atmosphere",
    },
    "aqua" => Item {
        name: "Aqua",
        description: "Mundane liquid",
    },
    "terra" => Item {
        name: "Terra",
        description: "Mundane solid",
    },
    "rock" => Item {
        name: "Rock",
        description: "Mundane solid",
    },
};
