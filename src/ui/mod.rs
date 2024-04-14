use bevy::prelude::*;

mod hud;
pub use hud::*;

mod save_ui;
pub use save_ui::*;

mod inventory_ui;
pub use inventory_ui::*;

mod cargo_shuttle_ui;
pub use cargo_shuttle_ui::*;

mod crafter_ui;
pub use crafter_ui::*;

mod interplanetary_freighter_ui;
pub use interplanetary_freighter_ui::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                setup_hud,
                update_hud,
                remove_windows_on_escape,
                spawn_save_ui,
            ),
        );
    }
}
