use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::CircleMeshBuilder, sprite::Material2d};

use crate::universe::random_polygon;

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
        let image = asset_server.load(sprite_loader.texture_path.clone());
        commands.entity(entity).insert(Sprite {
            image,
            color: sprite_loader.color,
            ..default()
        });
    }
}

// MATERIAL LOADER

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MaterialLoader<M: Material2d> {
    pub mesh_type: MeshType,
    pub material: M,
}

#[derive(Reflect)]
pub enum MeshType {
    Circle(f32),
    Rectangle(Vec2, Vec2),
    RandomPolygon(u64, f32),
}

impl Default for MeshType {
    fn default() -> Self {
        MeshType::Circle(1.)
    }
}

pub fn scan_material_loaders<M>(
    mut commands: Commands,
    mut materials: ResMut<Assets<M>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &MaterialLoader<M>), Added<MaterialLoader<M>>>,
) where
    M: Material2d,
{
    for (entity, material_loader) in query.iter() {
        let mesh = match material_loader.mesh_type {
            MeshType::Circle(radius) => {
                const ERR: f32 = 10.0;
                let vertices = (PI / (1. - ERR / radius).acos()).ceil() as u32;
                CircleMeshBuilder::new(radius, vertices).build()
            }
            MeshType::Rectangle(c1, c2) => Rectangle::from_corners(c1, c2).into(),
            MeshType::RandomPolygon(seed, avg_radius) => random_polygon(seed, avg_radius),
        };

        commands.entity(entity).insert((
            Mesh2d(meshes.add(mesh)),
            MeshMaterial2d(materials.add(material_loader.material.clone())),
        ));
    }
}

#[macro_export]
macro_rules! register_material {
    ($app:expr, $($material:ty),*) => {
        $(
            $app
                .add_plugins(Material2dPlugin::<$material>::default())
                .register_type::<$crate::MaterialLoader<$material>>()
                .add_systems(Update, $crate::scan_material_loaders::<$material>.in_set($crate::GameSet));
        )*
    };
}
