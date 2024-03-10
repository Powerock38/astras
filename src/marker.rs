use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

#[derive(Component)]
pub struct Marker;

pub fn spawn_marker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    position: Vec2,
) {
    let radius = 100.;

    let color = Color::rgb(0.7, 0.2, 0.3);

    let material = ColorMaterial::from(color);

    let mesh = RegularPolygon::new(radius, 4);

    let transform = Transform::from_translation(position.extend(0.));

    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(material),
            transform,
            ..default()
        })
        .insert(Marker);
}

pub fn update_marker(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_marker: Query<(Entity, &mut Transform), With<Marker>>,
) {
    let left = mouse_input.pressed(MouseButton::Left);
    let right = mouse_input.pressed(MouseButton::Right);

    if left || right {
        if q_marker.iter_mut().count() > 0 {
            for (entity, _) in q_marker.iter_mut() {
                commands.entity(entity).despawn();
            }
        }
    }

    if left {
        let (camera, camera_transform) = q_camera.single();
        let window = q_windows.single();
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            spawn_marker(commands, meshes, materials, world_position)
        }
    }
}
