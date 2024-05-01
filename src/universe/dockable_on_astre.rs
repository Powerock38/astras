use bevy::{prelude::*, utils::Uuid};

use crate::{
    buildings::PlacingLocation,
    universe::{Asteroid, Astre, SolarSystem},
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DockableOnAstre {
    on_astre: Option<Uuid>,
    instant_or_despawn: bool,
    location: PlacingLocation,
    adjust_z: bool,
}

impl DockableOnAstre {
    pub fn instant_location(location: PlacingLocation) -> Self {
        Self {
            instant_or_despawn: true,
            location,
            adjust_z: true,
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
    q_astre: Query<
        (Entity, &Astre, &GlobalTransform),
        (Without<DockableOnAstre>, Without<Asteroid>),
    >,
    q_solar_system: Query<(Entity, &GlobalTransform), With<SolarSystem>>,
) {
    let Some(solar_system) = q_solar_system.iter().next() else {
        return;
    };

    for (mut dockable, entity_dockable, mut transform, global_transform) in q_dockable.iter_mut() {
        let mut on_astre_option: Option<(Entity, Uuid, &GlobalTransform, f32)> = None;

        for (entity_astre, astre, astre_global_transform) in q_astre.iter() {
            let distance = global_transform.translation().truncate()
                - astre_global_transform.translation().truncate();
            let distance = distance.length();

            let can_dock = match dockable.location {
                PlacingLocation::Surface => distance < astre.surface_radius(),
                PlacingLocation::Atmosphere => {
                    distance < astre.surface_radius() + astre.atmosphere_radius()
                        && distance > astre.surface_radius()
                }
                PlacingLocation::SurfaceOrAtmosphere => {
                    distance < astre.surface_radius() + astre.atmosphere_radius()
                }
            };

            if can_dock {
                let astre_global_z = astre_global_transform.translation().z;

                if let Some((_, _, _, z)) = on_astre_option {
                    if z >= astre_global_z {
                        continue; // Already on a closer astre (on the z plane)
                    }
                }

                on_astre_option = Some((
                    entity_astre,
                    astre.uuid(),
                    astre_global_transform,
                    astre_global_z,
                ));
            }
        }

        // Found an astre to dock on
        if let Some((entity_astre, astre_uuid, astre_global_transform, _)) = on_astre_option {
            // If already docked on this astre, skip
            if let Some(entity_on_astre) = dockable.on_astre {
                if entity_on_astre == astre_uuid {
                    continue;
                }
            }

            // Entity goes in referential of astre
            *transform = global_transform.reparented_to(astre_global_transform);
            if dockable.adjust_z {
                transform.translation.z = 0.5;
            }
            commands.entity(entity_dockable).set_parent(entity_astre);

            // Dock
            dockable.on_astre = Some(astre_uuid);

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
                let (entity_solar_system, solar_system_global_transform) = solar_system;

                *transform = global_transform.reparented_to(solar_system_global_transform);
                if dockable.adjust_z {
                    transform.translation.z = 0.5;
                }
                commands
                    .entity(entity_dockable)
                    .set_parent(entity_solar_system);

                dockable.on_astre = None;
            }
        }
    }
}
