use crate::{astre::Astre, utils::ToReparent, SolarSystem};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct DockableOnAstre {
    on_astre: Option<Entity>,
}

pub fn update_dockable_on_astre(
    mut commands: Commands,
    mut q_dockable: Query<(&mut DockableOnAstre, Entity, &GlobalTransform)>,
    mut q_astre: Query<
        (Entity, &mut Astre, &Transform, &GlobalTransform),
        Without<DockableOnAstre>,
    >,
    q_solar_system: Query<Entity, With<SolarSystem>>,
) {
    for (mut dockable, entity_dockable, transform) in q_dockable.iter_mut() {
        let mut on_astre_option: Option<(Entity, f32)> = None;

        for (entity_astre, astre, astre_transform, astre_global_transform) in q_astre.iter_mut() {
            let distance = transform.translation().truncate()
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
            if let Some(entity_on_astre) = dockable.on_astre {
                if entity_on_astre == entity_astre {
                    continue;
                }
            }

            // In gravity field, entity stays in referential of astre
            commands.entity(entity_dockable).insert(ToReparent {
                new_parent: entity_astre,
            });

            dockable.on_astre = Some(entity_astre.clone());
        } else if dockable.on_astre.is_some() {
            // Not in gravity field, entity stays in referential of solar system
            let entity_solar_system = q_solar_system.single();
            commands.entity(entity_dockable).insert(ToReparent {
                new_parent: entity_solar_system,
            });

            dockable.on_astre = None;
        }
    }
}
