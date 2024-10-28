use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::{Crafter, BUILDINGS},
    items::{RecipeOutputs, RECIPES},
    ui::{build_item_ui, spawn_inventory_ui, HudWindow, HudWindowParent, UiButtonBundle},
};

pub fn scan_crafter_ui(mut commands: Commands, q_crafter: Query<Entity, Added<Crafter>>) {
    for entity in q_crafter.iter() {
        commands
            .entity(entity)
            .insert(On::<Pointer<Click>>::run(spawn_crafter_ui));
    }
}

pub fn spawn_crafter_ui(
    mut commands: Commands,
    listener: Listener<Pointer<Click>>,
    q_window_parent: Query<Entity, With<HudWindowParent>>,
    q_crafter: Query<&Crafter>,
    asset_server: Res<AssetServer>,
) {
    let parent = q_window_parent.single();
    let entity = listener.listener();
    let crafter = q_crafter.get(entity).unwrap();

    commands
        .entity(parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(HudWindow::default()).with_children(|c| {
                if !crafter.is_construction_site() {
                    // List recipes

                    c.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(50.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Start,
                            justify_content: JustifyContent::Start,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|c| {
                        for recipe in crafter.possible_recipes() {
                            let callback = {
                                let recipe = recipe.clone();
                                move |mut q_crafter: Query<&mut Crafter>| {
                                    let mut crafter = q_crafter.get_mut(entity).unwrap();
                                    crafter.set_recipe(recipe.clone());
                                }
                            };

                            c.spawn(UiButtonBundle::new(On::<Pointer<Click>>::run(callback)))
                                .with_children(|c| {
                                    if let Some(recipe) = RECIPES.get(recipe) {
                                        c.spawn(NodeBundle {
                                            style: Style {
                                                flex_direction: FlexDirection::Column,
                                                row_gap: Val::Px(5.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(
                                            |c| {
                                                c.spawn(NodeBundle {
                                                    style: Style {
                                                        align_items: AlignItems::Center,
                                                        flex_direction: FlexDirection::Row,
                                                        column_gap: Val::Px(5.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                                .with_children(|c| match recipe.outputs() {
                                                    RecipeOutputs::Items(outputs) => {
                                                        build_item_list_ui(c, outputs);
                                                    }
                                                    RecipeOutputs::Building(id) => {
                                                        build_building_ui(c, id, &asset_server);
                                                    }
                                                });

                                                c.spawn(NodeBundle {
                                                    style: Style {
                                                        align_items: AlignItems::Center,
                                                        flex_direction: FlexDirection::Row,
                                                        column_gap: Val::Px(5.0),
                                                        ..default()
                                                    },
                                                    ..default()
                                                })
                                                .with_children(|c| {
                                                    c.spawn(TextBundle::from_section(
                                                        "Needs",
                                                        TextStyle {
                                                            color: Color::srgb(0.9, 0.9, 0.9),
                                                            font_size: 18.0,
                                                            ..default()
                                                        },
                                                    ));

                                                    build_item_list_ui(c, recipe.inputs());
                                                });
                                            },
                                        );
                                    }
                                });
                        }
                    });
                }

                // Inventory
                spawn_inventory_ui(c, entity);
            });
        });
}

fn build_item_list_ui(c: &mut ChildBuilder, items: &[(&str, u32)]) {
    for (i, (id, quantity)) in items.iter().enumerate() {
        if i != 0 {
            c.spawn(TextBundle::from_section(
                "+",
                TextStyle {
                    color: Color::srgb(0.9, 0.9, 0.9),
                    font_size: 18.0,
                    ..default()
                },
            ));
        }

        build_item_ui(c, &(*id).to_string(), *quantity);
    }
}

pub fn build_building_ui(c: &mut ChildBuilder, id: &str, asset_server: &Res<AssetServer>) {
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
        let building = BUILDINGS.get(id).unwrap();
        let icon = asset_server.load(building.sprite_path());

        c.spawn(ImageBundle {
            style: Style {
                max_width: Val::Px(30.),
                height: Val::Px(30.),
                ..default()
            },
            image: UiImage::new(icon),
            ..default()
        });

        c.spawn(TextBundle::from_section(
            building.name,
            TextStyle {
                color: Color::srgb(0.9, 0.9, 0.9),
                font_size: 18.0,
                ..default()
            },
        ));
    });
}
