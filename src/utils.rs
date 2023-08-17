use bevy::prelude::*;

#[derive(Component)]
pub struct ToReparent {
    pub new_parent: Entity,
}

pub fn reparent_system(
    mut commands: Commands,
    mut targets: Query<(&mut Transform, Entity, &GlobalTransform, &ToReparent)>,
    transforms: Query<&GlobalTransform>,
) {
    for (mut transform, entity, initial, to_reparent) in targets.iter_mut() {
        if let Ok(parent_transform) = transforms.get(to_reparent.new_parent) {
            *transform = initial.reparented_to(parent_transform);
            commands
                .entity(entity)
                .remove::<ToReparent>()
                .set_parent(to_reparent.new_parent);

            // eprintln!("Reparented {:?} to {:?}", entity, to_reparent.new_parent);
        }
    }
}
