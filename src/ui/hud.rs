use bevy::prelude::*;

use crate::{ui::NotificationZone, universe::MainCamera};

#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct HudWindowParent;

#[derive(Component)]
pub struct HudWindowDependent;

#[derive(Component)]
#[require(
    Node(|| Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(10.)),
        ..default()
    }),
    BackgroundColor(|| BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)))
)]
pub struct HudWindow;

#[derive(Event)]
pub struct ClearUiEvent;

pub fn setup_hud(mut commands: Commands, camera: Single<Entity, Added<MainCamera>>) {
    commands
        .spawn((
            Hud,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            PickingBehavior::IGNORE,
            TargetCamera(*camera),
        ))
        .with_children(|c| {
            c.spawn((
                HudWindowParent,
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                PickingBehavior::IGNORE,
            ));

            c.spawn((
                NotificationZone,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(5.0),
                    right: Val::Px(5.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                PickingBehavior::IGNORE,
            ));
        });
}

pub fn clear_ui(
    _trigger: Trigger<ClearUiEvent>,
    mut commands: Commands,
    window_parent: Single<Entity, With<HudWindowParent>>,
    q_window_dependents: Query<Entity, With<HudWindowDependent>>,
) {
    commands.entity(*window_parent).despawn_descendants();

    for entity in &q_window_dependents {
        commands.entity(entity).despawn_recursive();
    }
}
