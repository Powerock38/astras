use bevy::{ecs::spawn::SpawnWith, prelude::*};

use crate::ui::{ClearUiEvent, UiButton};

pub fn build_building_header(name: &str) -> impl Bundle {
    let name = name.to_string();
    (
        Node {
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            border: UiRect::bottom(Val::Px(2.0)),
            padding: UiRect::bottom(Val::Px(5.0)),
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
        BorderColor(Color::WHITE),
        Children::spawn(SpawnWith(move |c: &mut ChildSpawner| {
            c.spawn((
                Text::new(name),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
            ));

            c.spawn((UiButton, children![Text::new("X")])).observe(
                move |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(ClearUiEvent);
                },
            );
        })),
    )
}
