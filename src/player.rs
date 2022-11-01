use bevy::prelude::*;

use crate::lib::controller::*;

pub fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn_bundle((
            Transform::default(),
            GlobalTransform::default(),
            Controller::default(),
            BodyTag,
        ))
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: assets.load("models/shiba/shiba.gltf#Scene0"),
                ..default()
            });
        });
}
