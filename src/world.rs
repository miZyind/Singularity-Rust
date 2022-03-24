use bevy::prelude::*;
use bevy_rapier3d::{
    physics::*,
    prelude::{
        ColliderFlags, ColliderShape, InteractionGroups, RigidBodyActivation, RigidBodyPosition,
        RigidBodyType,
    },
};

use crate::constants;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    let box_xz = 10.0;
    let box_y = 1.0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(box_xz, box_y, box_xz),
                Quat::IDENTITY,
                Vec3::ZERO,
            )),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5 * box_xz, 0.5 * box_y, 0.5 * box_xz).into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::all().with_memberships(constants::GROUND),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });
    // cubes
    let distance = 2.0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            activation: RigidBodyActivation::cannot_sleep().into(),
            body_type: RigidBodyType::Dynamic.into(),
            position: RigidBodyPosition {
                position: Vec3::new(distance, 1.0, distance).into(),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            activation: RigidBodyActivation::cannot_sleep().into(),
            body_type: RigidBodyType::Dynamic.into(),
            position: RigidBodyPosition {
                position: Vec3::new(distance, 1.0, -distance).into(),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            activation: RigidBodyActivation::cannot_sleep().into(),
            body_type: RigidBodyType::Dynamic.into(),
            position: RigidBodyPosition {
                position: Vec3::new(-distance, 1.0, distance).into(),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            activation: RigidBodyActivation::cannot_sleep().into(),
            body_type: RigidBodyType::Dynamic.into(),
            position: RigidBodyPosition {
                position: Vec3::new(-distance, 1.0, -distance).into(),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 0.5, 0.5).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete);
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..Default::default()
    });
}
