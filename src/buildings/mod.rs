use bevy::prelude::*;

mod building;
pub use building::*;

mod element_extractor;
pub use element_extractor::*;

mod warehouse;
pub use warehouse::*;

mod logistic_freight;
pub use logistic_freight::*;

mod crafter;
pub use crafter::*;

mod spaceport;
pub use spaceport::*;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                place_building,
                constructing_building,
                update_element_extractors,
                update_crafters.before(update_logistic_freights),
                update_logistic_freights,
            ),
        )
        .register_type::<ConstructingBuilding>()
        .register_type::<Building>()
        .register_type::<LogisticFreight>()
        .register_type::<Spaceport>()
        .register_type::<Warehouse>()
        .register_type::<ElementExtractor>()
        .register_type::<Crafter>();
    }
}
