use bevy::prelude::*;

use crate::items::{Inventory, ITEMS};

#[derive(Component)]
pub struct InventoryUI {
    entity: Entity,
}

pub fn spawn_inventory_ui(c: &mut ChildBuilder, entity: Entity) {
    c.spawn((
        InventoryUI { entity },
        NodeBundle {
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
        },
    ));
}

pub fn update_inventory_ui(
    mut commands: Commands,
    q_inventory_ui: Query<(Entity, &InventoryUI)>,
    q_inventories: Query<&Inventory>,
) {
    for (ui_entity, inventory_ui) in q_inventory_ui.iter() {
        let Ok(inventory) = q_inventories.get(inventory_ui.entity) else {
            continue;
        };

        commands
            .entity(ui_entity)
            .despawn_descendants()
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
}
