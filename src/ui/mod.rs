use bevy::prelude::*;

use crate::GameplaySet;

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
                    remove_windows_on_escape,
                    spawn_save_ui,
                    update_inventory_ui,
                    scan_crafter_ui,
                    scan_extractor_ui,
                    scan_logistic_freighter,
                )
                    .in_set(GameplaySet),
            ),
        );
    }
}
