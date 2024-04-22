use bevy::{
    prelude::*,
    sprite::{Material2d, Mesh2dHandle},
};

use crate::universe::circle_mesh;

// SPRITE LOADER

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SpriteLoader {
    pub texture_path: String,
    pub color: Color,
}

#[derive(Bundle, Default)]
pub struct HandleLoaderBundle<HandleLoader: Component> {
    pub loader: HandleLoader,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

pub fn scan_sprite_loaders(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &SpriteLoader), Added<SpriteLoader>>,
) {
    for (entity, sprite_loader) in query.iter() {
        let handle: Handle<Image> = asset_server.load(sprite_loader.texture_path.clone());
        commands.entity(entity).insert((
            handle,
            Sprite {
                color: sprite_loader.color,
                ..default()
            },
        ));
    }
}

// MATERIAL LOADER

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MaterialLoader<M: Material2d> {
    pub radius: f32,
    pub material: M,
}

pub fn scan_atres_material_loaders<M>(
    mut commands: Commands,
    mut materials: ResMut<Assets<M>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &MaterialLoader<M>), Added<MaterialLoader<M>>>,
) where
    M: Material2d,
{
    for (entity, material_loader) in query.iter() {
        let material = materials.add(material_loader.material.clone());
        let mesh = Mesh2dHandle::from(meshes.add(circle_mesh(material_loader.radius)));
        commands.entity(entity).insert((mesh, material));
    }
}
