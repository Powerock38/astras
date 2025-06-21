use std::sync::LazyLock;

use bevy::{color::palettes::css::*, platform::collections::HashMap, prelude::*};

use crate::{
    buildings::*,
    enum_map,
    items::{Element, ElementState, Inventory, Item, Recipe},
    universe::DockableOnAstre,
};

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

pub static ELEMENTS: LazyLock<HashMap<ItemId, Element>> = LazyLock::new(|| {
    HashMap::from([
        // Gases
        (ItemId::Aer, Element::new(ANTIQUE_WHITE, ElementState::Gas)),
        // Liquids
        (ItemId::Aqua, Element::new(BLUE, ElementState::Liquid)),
        // Solids
        (ItemId::Terra, Element::new(MAROON, ElementState::Solid)),
        (ItemId::Astrium, Element::new(SILVER, ElementState::Solid)),
        (
            ItemId::ElectroniteOre,
            Element::new(ORANGE_RED, ElementState::Solid),
        ),
        (
            ItemId::QuarkCrystal,
            Element::new(FUCHSIA, ElementState::Solid),
        ),
        // Plasmas
        (
            ItemId::Photonite,
            Element::new(YELLOW, ElementState::Plasma),
        ),
        (
            ItemId::Neutronite,
            Element::new(AQUAMARINE, ElementState::Plasma),
        ),
        (ItemId::Gravitonite, Element::new(RED, ElementState::Plasma)),
    ])
});

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

        // Buildings
        Quarry = Recipe::new_building(
            &[],
            BuildingId::Quarry,
            1.,
        ),

        LiquidExtractor = Recipe::new_building(
            &[],
            BuildingId::LiquidExtractor,
            1.,
        ),

        AtmosphereHarvester = Recipe::new_building(
            &[],
            BuildingId::AtmosphereHarvester,
            1.,
        ),

        PlasmaCatalyser = Recipe::new_building(
            &[],
            BuildingId::PlasmaCatalyser,
            1.,
        ),

        Warehouse = Recipe::new_building(
            &[],
            BuildingId::Warehouse,
            1.,
        ),

        CargoShuttle = Recipe::new_building(
            &[/*(ItemId::Astrium, 10), (ItemId::ComputingCore, 3), (ItemId::PlasmaFuel, 5)*/],
            BuildingId::CargoShuttle,
            3.,
        ),

        Spaceport = Recipe::new_building(
            &[],
            BuildingId::Spaceport,
            1.,
        ),

        InterplanetaryFreighter = Recipe::new_building(
            &[],
            BuildingId::InterplanetaryFreighter,
            1.,
        ),

        Foundry = Recipe::new_building(
            &[(ItemId::Astrium, 10)],
            BuildingId::Foundry,
            3.,
        ),

        Assembler = Recipe::new_building(
            &[],
            BuildingId::Assembler,
            1.,
        ),

        InterstellarGate = Recipe::new_building(
            &[],
            BuildingId::InterstellarGate,
            1.,
        ),
    }
}

enum_map! {
    BuildingId => BuildingData {
        Quarry = BuildingData {
            name: "Quarry",
            sprite_name: "quarry",
            location: LocationOnAstre::Surface,
            on_build: |c| {
                c.insert((Extractor::new_solid(), Inventory::new(1000)));
            },
        },

        LiquidExtractor = BuildingData {
            name: "Liquid Extractor",
            sprite_name: "quarry",
            location: LocationOnAstre::Surface,
            on_build: |c| {
                c.insert((Extractor::new_liquid(), Inventory::new(5000)));
            },
        },

        AtmosphereHarvester = BuildingData {
            name: "Atmosphere Harvester",
            sprite_name: "quarry",
            location: LocationOnAstre::Atmosphere,
            on_build: |c| {
                c.insert((Extractor::new_gas(), Inventory::new(10_000)));
            },
        },

        PlasmaCatalyser = BuildingData {
            name: "Plasma Catalyser",
            sprite_name: "quarry",
            location: LocationOnAstre::SurfaceOrAtmosphere,
            on_build: |c| {
                c.insert((Extractor::new_plasma(), Inventory::new(500)));
            },
        },

        Warehouse = BuildingData {
            name: "Warehouse",
            sprite_name: "warehouse",
            location: LocationOnAstre::Surface,
            on_build: |c| {
                c.insert((Warehouse, Inventory::new(100_000)));
            },
        },

        CargoShuttle = BuildingData {
            name: "Cargo Shuttle",
            sprite_name: "cargo_shuttle",
            location: LocationOnAstre::SurfaceOrAtmosphere,
            on_build: |c| {
                c.insert((LogisticFreight::new_planet(), Inventory::new(10_000)));
            },
        },

        Spaceport = BuildingData {
            name: "Spaceport",
            sprite_name: "spaceport",
            location: LocationOnAstre::CloseOrbit,
            on_build: |c| {
                c.insert((Spaceport, Inventory::new(1000)));
            },
        },

        InterplanetaryFreighter = BuildingData {
            name: "Interplanetary Freighter",
            sprite_name: "cargo_shuttle",
            location: LocationOnAstre::CloseOrbit,
            on_build: |c| {
                c.insert((LogisticFreight::new_solar_system(), Inventory::new(100_000), DockableOnAstre::default()));
            },
        },

        Foundry = BuildingData {
            name: "Foundry",
            sprite_name: "foundry",
            location: LocationOnAstre::Surface,
            on_build: |c| {
                c.insert((Crafter::new_crafter(vec![
                    RecipeId::SmeltElectroniteOre,
                    RecipeId::CraftPlasmaFuel,
                ]), Inventory::new(100)));
            },
        },

        Assembler = BuildingData {
            name: "Assembler",
            sprite_name: "assembler",
            location: LocationOnAstre::Surface,
            on_build: |c| {
                c.insert((Crafter::new_crafter(vec![
                    RecipeId::CraftComputingCore,
                    RecipeId::CargoShuttle,
                ]), Inventory::new(100)));
            },
        },

        InterstellarGate = BuildingData {
            name: "Interstellar Gate",
            sprite_name: "interstellar_gate",
            location: LocationOnAstre::CloseOrbit,
            on_build: |c| {
                c.insert(InterstellarGate);
            },
        },
    }
}
