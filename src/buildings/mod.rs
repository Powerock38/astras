use bevy::prelude::*;

mod building;
pub use building::*;

mod element_extractor;
pub use element_extractor::*;

mod warehouse;
pub use warehouse::*;

mod freighter;
pub use freighter::*;

mod crafter;
pub use crafter::*;

pub struct BuildingsPlugin;

impl Plugin for BuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                place_building,
                constructing_building,
                update_element_extractors,
                update_crafters.before(update_freighters),
                update_freighters,
            ),
        );
    }
}
