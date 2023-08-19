use dexterous_developer::hot_bevy_loader;

fn main() {
    hot_bevy_loader!(
        lib_astras::bevy_main,
        dexterous_developer::HotReloadOptions::default()
    );
}