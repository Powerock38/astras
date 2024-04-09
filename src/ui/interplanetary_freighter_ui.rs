use bevy::{
    prelude::*,
    render::{
        camera::{RenderTarget, Viewport},
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};
use bevy_mod_picking::prelude::*;

use crate::{
    buildings::LogisticFreight,
    items::{Inventory, LogisticProvider},
    ui::{spawn_inventory_ui, UIWindow, UIWindowDependent, UIWindowParent},
};

pub fn spawn_interplanetary_freighter_ui(
    mut commands: Commands,
    listener: Listener<Pointer<Click>>,
    mut images: ResMut<Assets<Image>>,
    q_ui_window_parent: Query<Entity, With<UIWindowParent>>,
    q_interplanetary_freighters: Query<(&LogisticFreight, &Inventory)>,
    q_providers: Query<Entity, With<LogisticProvider>>,
) {
    let parent = q_ui_window_parent.single();
    let (freight, inventory) = q_interplanetary_freighters
        .get(listener.listener())
        .unwrap();

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
                UIWindowDependent,
                Camera2dBundle {
                    camera: Camera {
                        viewport: Some(Viewport {
                            physical_position: UVec2::new(0, 0),
                            physical_size: UVec2::new(100, 100),
                            ..default()
                        }),
                        target: RenderTarget::Image(image_handle.clone()),
                        ..default()
                    },
                    ..default()
                },
            ));
        });

        Some(image_handle)
    } else {
        None
    };

    commands
        .entity(parent)
        .despawn_descendants()
        .with_children(|c| {
            c.spawn(UIWindow::default()).with_children(|c| {
                // Provider minimap

                if let Some(image_handle) = image_handle {
                    c.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(image_handle),
                    ));
                }

                // Inventory
                spawn_inventory_ui(c, inventory);
            });
        });
}
