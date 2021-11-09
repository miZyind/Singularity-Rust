use bevy::prelude::*;
use bevy_rapier3d::{physics::*, prelude::*};

use crate::{constants, lib::controller::*};

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle((
            Transform::default(),
            GlobalTransform::default(),
            Controller::default(),
            BodyTag,
        ))
        .insert_bundle(RigidBodyBundle {
            position: Vec3::new(0.0, 5.0, 0.0).into(),
            mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED_X
                | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z)
                .into(),
            activation: RigidBodyActivation::cannot_sleep(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 1.0, 0.5),
            mass_properties: ColliderMassProps::Density(1.0),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::all().with_memberships(constants::PLAYER),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("models/shiba/shiba.gltf#Scene0"));
        })
        .id();
}
