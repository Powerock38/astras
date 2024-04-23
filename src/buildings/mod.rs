use bevy::prelude::*;

use crate::GameplaySet;

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

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_building,
                update_extractors,
                update_logistic_freights,
                update_crafters,
            )
                .in_set(GameplaySet),
        )
        .register_type::<PlacingLocation>()
        .register_type::<ConstructionSite>()
        .register_type::<Building>()
        .register_type::<LogisticFreight>()
        .register_type::<LogisticJourneyWithTarget>()
        .register_type::<Option<LogisticJourneyWithTarget>>()
        .register_type::<Spaceport>()
        .register_type::<Warehouse>()
        .register_type::<Extractor>()
        .register_type::<Crafter>()
        .register_type::<CrafterRecipe>()
        .register_type::<Option<CrafterRecipe>>();
    }
}
