use bevy::prelude::*;

use crate::{
    data::{ItemId, ELEMENTS},
    items::{ElementState, Inventory, LogisticRequest},
    ui::UiButton,
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
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Row,
            ..default()
        },
    ));
}

pub fn update_inventory_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
            c.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Percent(5.0),
                ..default()
            })
            .with_children(|c| {
                c.spawn((
                    Text::new("Inventory:"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                ));

                if inventory.items().is_empty() {
                    c.spawn((
                        Text::new("Empty"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                    ));
                }

                for (id, quantity) in inventory.items() {
                    if ship.is_none() {
                        let callback =
                            item_transfer_callback(*id, *quantity, inventory_ui.entity, false);

                        c.spawn(UiButton).observe(callback).with_children(|c| {
                            build_item_ui(c, &asset_server, *id, *quantity);
                        });
                    } else {
                        build_item_ui(c, &asset_server, *id, *quantity);
                    }
                }
            });

            if let Some(logistic_request) = logistic_request {
                c.spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(5.0),
                    ..default()
                })
                .with_children(|c| {
                    c.spawn((
                        Text::new("Currently requesting:"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                    ));

                    for (id, quantity) in logistic_request.items() {
                        let callback =
                            item_transfer_callback(*id, *quantity, inventory_ui.entity, true);

                        c.spawn(UiButton).observe(callback).with_children(|c| {
                            build_item_ui(c, &asset_server, *id, *quantity);
                        });
                    }
                });
            }
        });
    }
}

pub fn build_item_ui(
    c: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    id: ItemId,
    quantity: u32,
) {
    let item = id.data();

    c.spawn(Node {
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(10.0),
        ..default()
    })
    .with_children(|c| {
        let (color, icon) = ELEMENTS
            .get(&id)
            .map_or((Color::WHITE.into(), "item"), |e| {
                (
                    e.color,
                    match e.state {
                        ElementState::Solid => "solid",
                        ElementState::Liquid => "liquid",
                        ElementState::Gas => "gas",
                        ElementState::Plasma => "plasma",
                    },
                )
            });

        let icon = asset_server.load(format!("icons/{icon}.png"));

        c.spawn((
            Node {
                width: Val::Px(30.),
                height: Val::Px(30.),
                ..default()
            },
            BackgroundColor(color.into()),
            ImageNode::new(icon),
        ));

        c.spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|c| {
            c.spawn((
                Text::new(format!("{} (x{})", item.name, quantity)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
            ));

            c.spawn((
                Text::new(item.description),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
            ));
        });
    });
}

fn item_transfer_callback(
    id: ItemId,
    quantity: u32,
    inventory_entity: Entity,
    from_ship: bool,
) -> impl FnMut(
    Trigger<Pointer<Click>>,
    Single<(&mut Inventory, &GlobalTransform), With<Ship>>,
    Query<(&mut Inventory, &GlobalTransform), Without<Ship>>,
) {
    move |_trigger: Trigger<Pointer<Click>>,
          q_ship: Single<(&mut Inventory, &GlobalTransform), With<Ship>>,
          mut q_inventory: Query<(&mut Inventory, &GlobalTransform), Without<Ship>>| {
        let (mut ship_inventory, ship_transform) = q_ship.into_inner();

        let Ok((mut inventory, transform)) = q_inventory.get_mut(inventory_entity) else {
            return;
        };

        if ship_transform
            .translation()
            .distance(transform.translation())
            < SHIP_ACTION_RANGE
        {
            if from_ship {
                ship_inventory.transfer_to(&mut inventory, id, quantity);
            } else {
                inventory.transfer_to(&mut ship_inventory, id, quantity);
            }
        }
    }
}
