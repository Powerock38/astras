use bevy::prelude::*;
use bevy_platform::collections::HashMap;

use crate::{
    data::{ItemId, ELEMENTS},
    items::{ElementState, Inventory, LogisticProvider, LogisticRequest, LogisticScope},
    ui::{HudWindow, HudWindowParent, UiButton},
    universe::{Ship, SHIP_ACTION_RANGE},
};

#[derive(Component)]
#[require(Node {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
    align_items: AlignItems::Start,
    justify_content: JustifyContent::SpaceBetween,
    flex_direction: FlexDirection::Row,
    ..default()
})]
pub struct InventoryUI {
    entity: Entity,
    just_added: bool,
    edit_logistic: bool,
}

impl InventoryUI {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            just_added: true,
            edit_logistic: false,
        }
    }

    pub fn with_edit_logistic(mut self) -> Self {
        self.edit_logistic = true;
        self
    }
}

pub fn update_inventory_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_inventories: Query<(
        &Inventory,
        Option<&LogisticRequest>,
        Option<&LogisticProvider>,
        Option<&Ship>,
    )>,
    mut q_inventory_ui: Query<(Entity, &mut InventoryUI)>,
    q_change_detection: Query<
        Entity,
        Or<(
            Changed<Inventory>,
            Changed<LogisticRequest>,
            Changed<LogisticProvider>,
        )>,
    >,
    mut removed_request: RemovedComponents<LogisticRequest>,
    mut removed_provider: RemovedComponents<LogisticProvider>,
) {
    for (ui_entity, mut inventory_ui) in &mut q_inventory_ui {
        let entity = inventory_ui.entity;

        if !inventory_ui.just_added
            && q_change_detection.get(entity).is_err()
            && !removed_request.read().any(|e| e == entity)
            && !removed_provider.read().any(|e| e == entity)
        {
            continue;
        }

        inventory_ui.just_added = false;

        let Ok((inventory, logistic_request, logistic_provider, ship)) = q_inventories.get(entity)
        else {
            continue;
        };

        let Ok(mut ec) = commands.get_entity(ui_entity) else {
            continue;
        };

        ec.despawn_related::<Children>().with_children(|c| {
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
                        let callback = item_transfer_callback(*id, *quantity, entity, false);

                        c.spawn((UiButton, build_item_ui(&asset_server, *id, *quantity)))
                            .observe(callback);
                    } else {
                        c.spawn(build_item_ui(&asset_server, *id, *quantity));
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
                        Text::new(format!(
                            "Currently requesting from {}:",
                            logistic_request.scope()
                        )),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                    ));

                    for (id, quantity) in logistic_request.items() {
                        let callback = item_transfer_callback(*id, *quantity, entity, true);

                        c.spawn((
                            UiButton,
                            children![build_item_ui(&asset_server, *id, *quantity)],
                        ))
                        .observe(callback);
                    }
                });
            }

            if inventory_ui.edit_logistic {
                if let Some(logistic_provider) = logistic_provider {
                    // Display Provider, +click to remove
                    c.spawn(UiButton)
                        .with_child(Text::new(format!(
                            "Currently exporting to the {}",
                            logistic_provider.scope()
                        )))
                        .observe(move |_: Trigger<Pointer<Click>>, mut commands: Commands| {
                            commands.entity(entity).remove::<LogisticProvider>();
                        });

                    // Build Logistic Request
                    let request_scope = logistic_provider.scope().opposite();
                    build_logistic_request_ui(c, entity, request_scope);
                } else {
                    // Set Provider (one option for each scope)
                    for scope in [LogisticScope::Planet, LogisticScope::SolarSystem] {
                        c.spawn(UiButton)
                            .with_child(Text::new(format!("Export to {scope}")))
                            .observe(move |_: Trigger<Pointer<Click>>, mut commands: Commands| {
                                commands.entity(entity).insert(LogisticProvider::new(scope));
                            });
                    }
                }
            }
        });
    }
}

pub fn build_logistic_request_ui(
    c: &mut ChildSpawnerCommands,
    entity: Entity,
    scope: LogisticScope,
) {
    c.spawn(UiButton)
        .with_child(Text::new("Change Request"))
        .observe(
            move |_: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  asset_server: Res<AssetServer>,
                  window_parent: Single<Entity, With<HudWindowParent>>| {
                commands.entity(*window_parent).with_children(|c| {
                    let mut ec = c.spawn(HudWindow);
                    let logistic_request_window_entity = ec.id();
                    ec.with_children(|c| {
                        c.spawn(UiButton).with_child(Text::new("Close")).observe(
                            move |_: Trigger<Pointer<Click>>, mut commands: Commands| {
                                commands.entity(logistic_request_window_entity).despawn();
                            },
                        );

                        for id in ItemId::ALL {
                            c.spawn((UiButton, children![build_item_ui(&asset_server, *id, 0)]))
                                .observe(
                                    move |trigger: Trigger<Pointer<Click>>,
                                          mut commands: Commands,
                                          mut query: Query<Option<&mut LogisticRequest>>| {
                                        let remove = trigger.button == PointerButton::Secondary;

                                        if let Some(mut request) =
                                            query.get_mut(entity).ok().flatten()
                                        {
                                            if remove {
                                                request.remove_item(*id, 1);
                                            } else {
                                                request.add_item(*id, 1);
                                            }
                                        } else if !remove {
                                            let mut request =
                                                LogisticRequest::new(HashMap::new(), scope);
                                            request.add_item(*id, 1);
                                            commands.entity(entity).insert(request);
                                        }
                                    },
                                );
                        }
                    });
                });
            },
        );
}

pub fn build_item_ui(asset_server: &Res<AssetServer>, id: ItemId, quantity: u32) -> impl Bundle {
    let item = id.data();

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

    (
        Node {
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Px(30.),
                    height: Val::Px(30.),
                    ..default()
                },
                BackgroundColor(color.into()),
                ImageNode::new(icon),
            ),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                children![
                    (
                        if quantity > 0 {
                            Text::new(format!("{} (x{quantity})", item.name))
                        } else {
                            Text::new(item.name)
                        },
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                    ),
                    (
                        Text::new(item.description),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                    )
                ]
            )
        ],
    )
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
