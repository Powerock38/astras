use bevy::prelude::*;

use crate::ui::{ClearUiEvent, UiButton};

pub fn spawn_building_header(c: &mut ChildBuilder, entity: Entity, name: &str) {
    c.spawn((
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
    ))
    .with_children(|c| {
        c.spawn((
            Text::new(name),
            TextFont {
                font_size: 18.0,
                ..default()
            },
        ));

        c.spawn(UiButton)
            .observe(
                move |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(ClearUiEvent);
                    commands.entity(entity).despawn_recursive();
                },
            )
            .with_children(|c| {
                c.spawn(Text::new("X"));
            });
    });
}
