use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    items::{Inventory, LogisticRequest, ELEMENTS, ITEMS},
    ui::UiButtonBundle,
    universe::{Ship, SHIP_ACTION_RANGE},
};

#[derive(Component)]
pub struct InventoryUI {
    entity: Entity,
    just_added: bool,
}

pub fn spawn_inventory_ui(c: &mut ChildBuilder, entity: Entity) {
    c.spawn((
        InventoryUI {
            entity,
            just_added: true,
        },
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
    ));
}

pub fn update_inventory_ui(
    mut commands: Commands,
    q_inventories: Query<(&Inventory, Option<&LogisticRequest>, Option<&Ship>)>,
    mut q_inventory_ui: Query<(Entity, &mut InventoryUI)>,
    q_change_detection: Query<Entity, Or<(Changed<Inventory>, Changed<LogisticRequest>)>>,
    mut q_removal_detection: RemovedComponents<LogisticRequest>,
) {
    for (ui_entity, mut inventory_ui) in &mut q_inventory_ui {
        if !inventory_ui.just_added
            && q_change_detection.get(inventory_ui.entity).is_err()
            && !q_removal_detection.read().any(|e| e == inventory_ui.entity)
        {
            continue;
        }

        inventory_ui.just_added = false;

        let Ok((inventory, logistic_request, ship)) = q_inventories.get(inventory_ui.entity) else {
            continue;
        };

        let Some(mut ec) = commands.get_entity(ui_entity) else {
            continue;
        };

        ec.despawn_descendants().with_children(|c| {
            c.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(5.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|c| {
                for (id, quantity) in inventory.items() {
                    if ship.is_none() {
                        let callback = item_transfer_callback(
                            id.clone(),
                            *quantity,
                            inventory_ui.entity,
                            false,
                        );

                        c.spawn(UiButtonBundle::new(On::<Pointer<Click>>::run(callback)))
                            .with_children(|c| {
                                build_item_ui(c, id, *quantity);
                            });
                    } else {
                        build_item_ui(c, id, *quantity);
                    }
                }
            });

            if let Some(logistic_request) = logistic_request {
                c.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Start,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Percent(5.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|c| {
                    c.spawn(TextBundle::from_section(
                        "This building needs:",
                        TextStyle {
                            color: Color::rgb(0.9, 0.9, 0.9),
                            font_size: 24.0,
                            ..default()
                        },
                    ));

                    for (id, quantity) in logistic_request.items() {
                        let callback = item_transfer_callback(
                            id.clone(),
                            *quantity,
                            inventory_ui.entity,
                            true,
                        );

                        c.spawn(UiButtonBundle::new(On::<Pointer<Click>>::run(callback)))
                            .with_children(|c| {
                                build_item_ui(c, id, *quantity);
                            });
                    }
                });
            }
        });
    }
}

pub fn build_item_ui(c: &mut ChildBuilder, id: &String, quantity: u32) {
    let item = ITEMS.get(id).unwrap();
    c.spawn(NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            ..default()
        },
        ..default()
    })
    .with_children(|c| {
        let color = ELEMENTS.get(id).map_or(Color::WHITE, |e| e.color);

        c.spawn(NodeBundle {
            style: Style {
                width: Val::Px(30.),
                height: Val::Px(30.),
                ..default()
            },
            background_color: color.into(),
            ..default()
        });

        c.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },
            ..default()
        })
        .with_children(|c| {
            c.spawn(TextBundle::from_section(
                format!("{} (x{})", item.name, quantity),
                TextStyle {
                    color: Color::rgb(0.9, 0.9, 0.9),
                    font_size: 18.0,
                    ..default()
                },
            ));

            c.spawn(TextBundle::from_section(
                item.description,
                TextStyle {
                    color: Color::rgb(0.9, 0.9, 0.9),
                    font_size: 12.0,
                    ..default()
                },
            ));
        });
    });
}

fn item_transfer_callback(
    id: String,
    quantity: u32,
    inventory_entity: Entity,
    from_ship: bool,
) -> impl FnMut(
    Query<(&mut Inventory, &GlobalTransform), With<Ship>>,
    Query<(&mut Inventory, &GlobalTransform), Without<Ship>>,
) {
    move |mut q_ship: Query<(&mut Inventory, &GlobalTransform), With<Ship>>,
          mut q_inventory: Query<(&mut Inventory, &GlobalTransform), Without<Ship>>| {
        let Some((mut ship_inventory, ship_transform)) = q_ship.iter_mut().next() else {
            return;
        };

        let Ok((mut inventory, transform)) = q_inventory.get_mut(inventory_entity) else {
            return;
        };

        if ship_transform
            .translation()
            .distance(transform.translation())
            < SHIP_ACTION_RANGE
        {
            if from_ship {
                ship_inventory.transfer_to(&mut inventory, id.clone(), quantity);
            } else {
                inventory.transfer_to(&mut ship_inventory, id.clone(), quantity);
            }
        }
    }
}
