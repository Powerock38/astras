use bevy::{
    core_pipeline::clear_color::ClearColorConfig, input::mouse::MouseWheel, prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::{astre::Astre, utils::ToReparent, SolarSystem};

#[derive(Component)]
pub struct Ship {
    speed: f32,
    on_astre: Option<Entity>,
}

pub fn setup_ship(
    c: &mut ChildBuilder,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let position = Vec2::new(0., 0.);

    c.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(), // Triangle
        material: materials.add(ColorMaterial::from(Color::GOLD)),
        transform: Transform::from_translation(position.extend(1.)),
        ..default()
    })
    .insert(Ship {
        speed: 3000., // pixels per second
        on_astre: None,
    })
    .with_children(|c| {
        c.spawn(Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 100.)),
            ..default()
        });
    });
}

pub fn update_ship(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Ship, &mut Transform)>,
) {
    for (ship, mut transform) in query.iter_mut() {
        let mut movement = Vec3::new(0., 0., 0.);

        if keyboard_input.pressed(KeyCode::Left) {
            movement.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            movement.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            movement.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            movement.y -= 1.;
        }

        transform.translation += movement * ship.speed * time.delta_seconds();
    }
}

pub fn update_ship_on_astre(
    mut commands: Commands,
    mut q_ship: Query<(&mut Ship, Entity, &GlobalTransform)>,
    mut q_astre: Query<(Entity, &mut Astre, &Transform, &GlobalTransform), Without<Ship>>,
    q_solar_system: Query<Entity, With<SolarSystem>>,
) {
    for (mut ship, entity_ship, ship_global_transform) in q_ship.iter_mut() {
        let mut on_astre_option: Option<(Entity, f32)> = None;

        for (entity_astre, astre, astre_transform, astre_global_transform) in q_astre.iter_mut() {
            let distance = ship_global_transform.translation().truncate()
                - astre_global_transform.translation().truncate();
            let distance = distance.length();

            if distance < astre.radius + astre.mass {
                if let Some((_, z)) = on_astre_option {
                    if z <= astre_transform.translation.z {
                        continue; // Already on a closer astre
                    }
                }

                on_astre_option = Some((entity_astre, astre_transform.translation.z));
            }
        }

        if let Some((entity_astre, _)) = on_astre_option {
            if let Some(entity_on_astre) = ship.on_astre {
                if entity_on_astre == entity_astre {
                    continue;
                }
            }

            // In gravity field, ship stays in referential of astre
            commands.entity(entity_ship).insert(ToReparent {
                new_parent: entity_astre,
            });

            ship.on_astre = Some(entity_astre.clone());
        } else if ship.on_astre.is_some() {
            // Not in gravity field, ship stays in referential of solar system
            let entity_solar_system = q_solar_system.single();
            commands.entity(entity_ship).insert(ToReparent {
                new_parent: entity_solar_system,
            });

            ship.on_astre = None;
        }
    }
}

pub fn update_camera(
    time: Res<Time>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for scroll in scroll_evr.iter() {
        for mut projection in query.iter_mut() {
            projection.scale *= 1. + 2. * scroll.y * time.delta_seconds();
        }
    }
}
