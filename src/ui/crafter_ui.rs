use bevy::prelude::*;

use crate::{
    buildings::Crafter,
    data::{BuildingId, ItemId},
    items::RecipeOutputs,
    ui::{build_building_header, build_item_ui, HudWindow, HudWindowParent, InventoryUI, UiButton},
};

pub fn scan_crafter_ui(mut commands: Commands, q_crafters: Query<Entity, Added<Crafter>>) {
    for entity in &q_crafters {
        commands.entity(entity).observe(spawn_crafter_ui);
    }
}

fn spawn_crafter_ui(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_parent: Single<Entity, With<HudWindowParent>>,
    q_crafter: Query<&Crafter>,
) {
    let entity = trigger.target();
    let crafter = q_crafter.get(entity).unwrap();

    commands
        .entity(*window_parent)
        .despawn_related::<Children>()
        .with_children(|c| {
            c.spawn(HudWindow).with_children(|c| {
                let name = if crafter.is_construction_site() { "Construction site" } else { "Crafter" };
                c.spawn(build_building_header(name));

                if !crafter.is_construction_site() {
                    // List recipes

                    c.spawn(Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Start,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        ..default()
                    })
                    .with_children(|c| {
                        c.spawn((
                            Text::new("Recipes:"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                        ));

                        for recipe in crafter.possible_recipes() {
                            let callback = {
                                let recipe = *recipe;
                                move |_trigger: Trigger<Pointer<Click>> ,mut q_crafter: Query<&mut Crafter>| {
                                    let mut crafter = q_crafter.get_mut(entity).unwrap();
                                    crafter.set_recipe(recipe);
                                }
                            };

                            c.spawn(UiButton)
                            .observe(callback)
                            .with_children(|c| {
                                let recipe = recipe.data();

                                c.spawn(Node {
                                        flex_direction: FlexDirection::Column,
                                        row_gap: Val::Px(5.0),
                                        ..default()
                                })
                                .with_children(|c| {
                                    c.spawn(Node {
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Row,
                                            column_gap: Val::Px(5.0),
                                            ..default()
                                    })
                                    .with_children(
                                        |c| match recipe.outputs() {
                                            RecipeOutputs::Items(outputs) => {
                                                build_item_list_ui(c, &asset_server, outputs);
                                            }
                                            RecipeOutputs::Building(id) => {
                                                c.spawn(build_building_ui(id, &asset_server));
                                            }
                                        },
                                    );

                                    c.spawn(Node {
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Row,
                                            column_gap: Val::Px(5.0),
                                            ..default()
                                    })
                                    .with_children(|c| {
                                            c.spawn((Text::new("Needs"),
                                                TextFont {
                                                    font_size: 18.0,
                                                    ..default()
                                                },
                                            ));

                                            build_item_list_ui(c, &asset_server, recipe.inputs());
                                        },
                                    );
                                });
                            });
                        }
                    });
                }

                // Inventory
                c.spawn(InventoryUI::new(entity));
            });
        });
}

fn build_item_list_ui(
    c: &mut ChildSpawnerCommands,
    asset_server: &Res<AssetServer>,
    items: &[(ItemId, u32)],
) {
    for (i, (id, quantity)) in items.iter().enumerate() {
        if i != 0 {
            c.spawn((
                Text::new("+"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
            ));
        }

        c.spawn(build_item_ui(asset_server, *id, *quantity));
    }
}

pub fn build_building_ui(id: BuildingId, asset_server: &Res<AssetServer>) -> impl Bundle {
    let building = id.data();
    let icon = asset_server.load(building.sprite_path());

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
                    max_width: Val::Px(30.),
                    height: Val::Px(30.),
                    ..default()
                },
                ImageNode::new(icon),
            ),
            (
                Text::new(building.name),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
            )
        ],
    )
}
