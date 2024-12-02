use bevy::{
    prelude::*,
    render::{
        camera::{RenderTarget, Viewport},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};

use crate::{
    buildings::LogisticFreight,
    items::{LogisticProvider, LogisticScope},
    ui::{spawn_inventory_ui, HudWindow, HudWindowDependent, HudWindowParent},
};

pub fn scan_logistic_freighter(
    mut commands: Commands,
    q_cargo_shuttles: Query<(Entity, &LogisticFreight), Added<LogisticFreight>>,
) {
    for (entity, logistic_freight) in &q_cargo_shuttles {
        match logistic_freight.scope() {
            LogisticScope::Planet => {
                commands.entity(entity).observe(spawn_cargo_shuttle_ui);
            }

            LogisticScope::SolarSystem => {
                commands
                    .entity(entity)
                    .observe(spawn_interplanetary_freighter_ui);
            }
        }
    }
}

pub fn spawn_cargo_shuttle_ui(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    window_parent: Single<Entity, With<HudWindowParent>>,
) {
    let entity = trigger.entity();

    commands
        .entity(*window_parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(HudWindow).with_children(|c| {
                spawn_inventory_ui(c, entity);
            });
        });
}

pub fn spawn_interplanetary_freighter_ui(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    window_parent: Single<Entity, With<HudWindowParent>>,
    q_interplanetary_freighters: Query<&LogisticFreight>,
    q_providers: Query<Entity, With<LogisticProvider>>,
) {
    let entity = trigger.entity();
    let freight = q_interplanetary_freighters.get(entity).unwrap();

    let image_handle = if let Some(logistic_journey) = freight.logistic_journey() {
        let provider_entity = q_providers.get(logistic_journey.provider()).unwrap();

        let size = Extent3d {
            width: 100,
            height: 100,
            ..default()
        };

        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                view_formats: &[TextureFormat::Bgra8UnormSrgb],
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
            },
            ..default()
        };
        //FIXME: image is never deleted
        image.resize(size);
        let image_handle = images.add(image);

        commands.entity(provider_entity).with_children(|c| {
            c.spawn((
                HudWindowDependent,
                Camera2d,
                Camera {
                    viewport: Some(Viewport {
                        physical_position: UVec2::new(0, 0),
                        physical_size: UVec2::new(100, 100),
                        ..default()
                    }),
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
            ));
        });

        Some(image_handle)
    } else {
        None
    };

    commands
        .entity(*window_parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(HudWindow).with_children(|c| {
                // Provider minimap

                if let Some(image_handle) = image_handle {
                    c.spawn((
                        Node {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                        ImageNode::new(image_handle),
                    ));
                }

                // Inventory
                spawn_inventory_ui(c, entity);
            });
        });
}
