use bevy::prelude::*;

use crate::SolarSystemSet;

mod building;
mod crafter;
mod extractor;
mod interstellar_gate;
mod logistic_freight;
mod spaceport;
mod warehouse;

pub use building::*;
pub use crafter::*;
pub use extractor::*;
pub use interstellar_gate::*;
pub use logistic_freight::*;
pub use spaceport::*;
pub use warehouse::*;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_building,
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
        .add_observer(observe_register_freight);
    }
}
