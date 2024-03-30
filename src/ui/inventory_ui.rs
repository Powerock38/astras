use bevy::prelude::*;

use crate::items::{Inventory, ITEMS};

pub fn spawn_inventory_ui(c: &mut ChildBuilder, inventory: &Inventory) {
    c.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Percent(5.0),
            ..default()
        },
        ..default()
    })
    .with_children(|c| {
        for (id, quantity) in inventory.items() {
            let item = ITEMS.get(id).unwrap();
            c.spawn(TextBundle::from_section(
                format!("{} (x{})\n{}", item.name, quantity, item.description),
                TextStyle {
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        }
    });
}
