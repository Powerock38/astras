use bevy::prelude::*;

use crate::SolarSystemSet;

mod building_ui;
mod buttons;
mod crafter_ui;
mod extractor_ui;
mod hud;
mod inventory_ui;
mod logistic_freighter_ui;
mod notification;
mod save_load_ui;
mod ship_ui;
mod spaceport_ui;

pub use building_ui::*;
pub use buttons::*;
pub use crafter_ui::*;
pub use extractor_ui::*;
pub use hud::*;
pub use inventory_ui::*;
pub use logistic_freighter_ui::*;
pub use notification::*;
pub use save_load_ui::*;
pub use ship_ui::*;
pub use spaceport_ui::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_ui_buttons,
                (
                    setup_hud,
                    clear_ui_or_spawn_ship_ui,
                    spawn_save_ui,
                    update_inventory_ui.after(clear_ui_or_spawn_ship_ui),
                    scan_crafter_ui,
                    scan_extractor_ui,
                    scan_spaceport_ui,
                    scan_logistic_freighter,
                    update_notifications,
                )
                    .in_set(SolarSystemSet),
            ),
        )
        .add_observer(clear_ui)
        .add_observer(observe_notifications);
    }
}
