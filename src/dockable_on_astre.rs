use crate::{astres::Astre, PlacingLocation, SolarSystem};
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct DockableOnAstre {
    on_astre: Option<Entity>,
    instant_or_despawn: bool,
    location: PlacingLocation,
}

impl DockableOnAstre {
    pub fn instant_location(location: PlacingLocation) -> Self {
        Self {
            instant_or_despawn: true,
            location,
            ..default()
        }
    }

    pub fn is_docked(&self) -> bool {
        self.on_astre.is_some()
    }
}

pub fn update_dockable_on_astre(
    mut commands: Commands,
    mut q_dockable: Query<(
        &mut DockableOnAstre,
        Entity,
        &mut Transform,
        &GlobalTransform,
    )>,
    q_astre: Query<(Entity, &Astre, &Transform, &GlobalTransform), Without<DockableOnAstre>>,
    q_solar_system: Query<(Entity, &GlobalTransform), With<SolarSystem>>,
) {
    for (mut dockable, entity_dockable, mut transform, global_transform) in q_dockable.iter_mut() {
        let mut on_astre_option: Option<(Entity, &GlobalTransform, f32)> = None;

        for (entity_astre, astre, astre_transform, astre_global_transform) in q_astre.iter() {
            let distance = global_transform.translation().truncate()
                - astre_global_transform.translation().truncate();
            let distance = distance.length();

            let can_dock = match dockable.location {
                PlacingLocation::Surface => distance < astre.surface_radius,
                PlacingLocation::Atmosphere => {
                    distance < astre.surface_radius + astre.atmosphere_radius
                        && distance > astre.surface_radius
                }
                PlacingLocation::SurfaceOrAtmosphere => {
                    distance < astre.surface_radius + astre.atmosphere_radius
                }
            };

            if can_dock {
                if let Some((_, _, z)) = on_astre_option {
                    if z <= astre_transform.translation.z {
                        continue; // Already on a closer astre (on the z plane)
                    }
                }

                on_astre_option = Some((
                    entity_astre,
                    astre_global_transform,
                    astre_transform.translation.z,
                ));
            }
        }

        // Found an astre to dock on
        if let Some((entity_astre, astre_global_transform, _)) = on_astre_option {
            // If already docked on this astre, skip
            if let Some(entity_on_astre) = dockable.on_astre {
                if entity_on_astre == entity_astre {
                    continue;
                }
            }

            // Entity goes in referential of astre
            *transform = global_transform.reparented_to(astre_global_transform);
            commands.entity(entity_dockable).set_parent(entity_astre);

            // Dock
            dockable.on_astre = Some(entity_astre.clone());

            // If dockable is instant_or_despawn and we found an astre, remove the component
            if dockable.instant_or_despawn {
                println!(
                    "Docking forever {:?} on astre {:?}!",
                    entity_dockable, entity_astre
                );
                commands.entity(entity_dockable).remove::<DockableOnAstre>();
                continue;
            }
        } else {
            // If dockable is instant_or_despawn and we didn't find an astre, remove the Entity
            if dockable.instant_or_despawn {
                println!("Despawning {:?}!", entity_dockable);
                commands.entity(entity_dockable).despawn();
                continue;
            }

            if dockable.on_astre.is_some() {
                // Entity left astre, goes in referential of solar system
                let (entity_solar_system, solar_system_global_transform) = q_solar_system.single();

                *transform = global_transform.reparented_to(solar_system_global_transform);
                commands
                    .entity(entity_dockable)
                    .set_parent(entity_solar_system);

                dockable.on_astre = None;
            }
        }
    }
}
