use bevy::prelude::*;

use crate::SolarSystemSet;

mod building;
pub use building::*;

mod extractor;
pub use extractor::*;

mod warehouse;
pub use warehouse::*;

mod logistic_freight;
pub use logistic_freight::*;

mod crafter;
pub use crafter::*;

mod spaceport;
pub use spaceport::*;

mod interstellar_gate;
pub use interstellar_gate::*;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_building.param_warn_once(),
                draw_placing_zones,
                update_extractors,
                update_logistic_freights,
                update_logistic_freights_movement.after(update_logistic_freights),
                update_crafters,
                add_highlight_selection,
            )
                .in_set(SolarSystemSet),
        )
        .add_observer(observe_unregister_freight)
        .add_observer(observe_freight_inventory_transfer)
        .add_observer(observe_register_freight)
        .register_type::<BuildingHighlight>()
        .register_type::<LogisticFreight>()
        .register_type::<Spaceport>()
        .register_type::<Warehouse>()
        .register_type::<Extractor>()
        .register_type::<InterstellarGate>()
        .register_type::<Crafter>();
    }
}
