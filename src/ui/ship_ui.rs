use bevy::prelude::*;

use crate::{
    buildings::PlacingBuilding,
    data::BuildingId,
    ui::{build_building_ui, ClearUiEvent, HudWindow, HudWindowParent, InventoryUI, UiButton},
    universe::Ship,
};

pub fn clear_ui_or_spawn_ship_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window_parent: Single<Entity, With<HudWindowParent>>,
    q_children: Query<Entity, With<Children>>,
    ship: Single<Entity, With<Ship>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Escape, KeyCode::KeyE]) {
        if q_children.get(*window_parent).is_ok() {
            commands.trigger(ClearUiEvent);
        } else {
            // SPAWN SHIP UI

            commands.entity(*window_parent).with_children(|c| {
                c.spawn(HudWindow).with_children(|c| {
                    c.spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::SpaceBetween,
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(10.0),
                        ..default()
                    })
                    .with_children(|c| {
                        c.spawn(InventoryUI::new(*ship));

                        c.spawn(Node {
                            width: Val::Percent(50.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::End,
                            justify_content: JustifyContent::Start,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            ..default()
                        })
                        .with_children(|c| {
                            for building_id in BuildingId::ALL {
                                let callback =
                                    |_pointer_click: On<Pointer<Click>>, mut commands: Commands| {
                                        commands.insert_resource(PlacingBuilding(*building_id));
                                    };

                                c.spawn((
                                    UiButton,
                                    children![build_building_ui(*building_id, &asset_server)],
                                ))
                                .observe(callback);
                            }
                        });
                    });
                });
            });
        }
    }
}
