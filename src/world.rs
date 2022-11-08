use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    let plane_size = 10.0;
    let plane_height = 1.0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(plane_size, plane_height, plane_size),
                Quat::IDENTITY,
                Vec3::ZERO,
            )),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5));
    // cubes
    let distance = 2.0;
    let collider = Collider::cuboid(0.5, 0.5, 0.5);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        })
        .insert_bundle((
            Transform::from_xyz(distance, 5.0, distance),
            RigidBody::Dynamic,
            collider.clone(),
        ));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        })
        .insert_bundle((
            Transform::from_xyz(distance, 5.0, -distance),
            RigidBody::Dynamic,
            collider.clone(),
        ));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        })
        .insert_bundle((
            Transform::from_xyz(-distance, 5.0, distance),
            RigidBody::Dynamic,
            collider.clone(),
        ));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        })
        .insert_bundle((
            Transform::from_xyz(-distance, 5.0, -distance),
            RigidBody::Dynamic,
            collider.clone(),
        ));
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
}
