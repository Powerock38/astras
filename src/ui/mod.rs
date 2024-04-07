use bevy::prelude::*;

mod hud;
pub use hud::*;

mod inventory_ui;
pub use inventory_ui::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .add_systems(Update, (update_hud, remove_windows_on_escape));
    }
}
