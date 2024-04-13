use bevy::prelude::*;

mod element;
pub use element::*;

mod item;
pub use item::*;

mod inventory;
pub use inventory::*;

mod recipe;
pub use recipe::*;

mod logistic;
pub use logistic::*;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Inventory>()
            .register_type::<LogisticRequest>()
            .register_type::<LogisticProvider>();
    }
}
