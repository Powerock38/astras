use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

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

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
}
