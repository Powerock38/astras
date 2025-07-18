use bevy::prelude::*;

use super::ActiveSolarSystem;
use crate::{
    buildings::LocationOnAstre,
    universe::{Asteroid, Astre},
};

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct DockableOnAstre {
    pub on_astre: bool,
    instant_or_despawn: bool,
    location: LocationOnAstre,
    adjust_z: bool,
}

impl DockableOnAstre {
    pub fn instant_location(location: LocationOnAstre) -> Self {
        Self {
            instant_or_despawn: true,
            location,
            adjust_z: true,
            ..default()
        }
    }
}

pub fn update_dockable_on_astre(
    mut commands: Commands,
    q_solar_system: Single<(Entity, &GlobalTransform), With<ActiveSolarSystem>>,
    mut q_dockable: Query<(
        Entity,
        &mut DockableOnAstre,
        Option<&ChildOf>,
        &mut Transform,
        &GlobalTransform,
    )>,
    q_astres: Query<
        (Entity, &Astre, &GlobalTransform, &InheritedVisibility),
        (Without<DockableOnAstre>, Without<Asteroid>),
    >,
) {
    let (entity_solar_system, solar_system_global_transform) = q_solar_system.into_inner();

    for (entity_dockable, mut dockable, dockable_parent, mut transform, global_transform) in
        &mut q_dockable
    {
        let mut on_astre_option: Option<(Entity, &GlobalTransform, f32)> = None;

        for (entity_astre, astre, astre_global_transform, _) in
            q_astres.iter().filter(|(_, _, _, v)| v.get())
        {
            let distance = global_transform.translation().truncate()
                - astre_global_transform.translation().truncate();
            let distance = distance.length();

            let can_dock = match dockable.location {
                LocationOnAstre::Surface => distance < astre.surface_radius(),
                LocationOnAstre::Atmosphere => {
                    distance < astre.atmosphere_radius() && distance > astre.surface_radius()
                }
                LocationOnAstre::SurfaceOrAtmosphere => distance < astre.atmosphere_radius(),
                LocationOnAstre::CloseOrbit => {
                    distance < astre.close_orbit_radius() && distance > astre.atmosphere_radius()
                }
                LocationOnAstre::Anywhere => distance < astre.close_orbit_radius(),
            };

            if can_dock {
                let astre_global_z = astre_global_transform.translation().z;

                if let Some((_, _, z)) = on_astre_option {
                    if z >= astre_global_z {
                        continue; // Already on a closer astre (on the z plane)
                    }
                }

                on_astre_option = Some((entity_astre, astre_global_transform, astre_global_z));
            }
        }

        // Found an astre to dock on
        if let Some((entity_astre, astre_global_transform, _)) = on_astre_option {
            // If already docked on this astre, skip
            if dockable.on_astre {
                if let Some(dockable_child_of) = dockable_parent {
                    if dockable_child_of.parent() == entity_astre {
                        continue;
                    }
                }
            }

            // Entity goes in referential of astre
            *transform = global_transform.reparented_to(astre_global_transform);
            if dockable.adjust_z {
                transform.translation.z = 0.5;
            }
            commands
                .entity(entity_dockable)
                .insert(ChildOf(entity_astre));

            // Dock
            dockable.on_astre = true;

            // If dockable is instant_or_despawn and we found an astre, remove the component
            if dockable.instant_or_despawn {
                debug!("Docking forever {entity_dockable:?} on astre {entity_astre:?}!");
                commands.entity(entity_dockable).remove::<DockableOnAstre>();
                continue;
            }

            debug!("Docking {entity_dockable:?} on astre {entity_astre:?}!");
        } else {
            // If dockable is instant_or_despawn and we didn't find an astre, remove the Entity
            if dockable.instant_or_despawn {
                debug!("Despawning {entity_dockable:?}!");
                commands.entity(entity_dockable).despawn();
                continue;
            }

            if dockable.on_astre {
                // Entity left astre, goes in referential of solar system

                *transform = global_transform.reparented_to(solar_system_global_transform);
                if dockable.adjust_z {
                    transform.translation.z = 0.5;
                }
                commands
                    .entity(entity_dockable)
                    .insert(ChildOf(entity_solar_system));

                dockable.on_astre = false;

                debug!("Undocking {entity_dockable:?}!");
            }
        }
    }
}
