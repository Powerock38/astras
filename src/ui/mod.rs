use bevy::prelude::*;

use crate::SolarSystemSet;

mod buttons;
pub use buttons::*;

mod hud;
pub use hud::*;

mod save_load_ui;
pub use save_load_ui::*;

mod inventory_ui;
pub use inventory_ui::*;

mod crafter_ui;
pub use crafter_ui::*;

mod extractor_ui;
pub use extractor_ui::*;

mod spaceport_ui;
pub use spaceport_ui::*;

mod logistic_freighter_ui;
pub use logistic_freighter_ui::*;

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
                )
                    .in_set(SolarSystemSet),
            ),
        );
    }
}
