use crate::{constants, lib::look::LookCamera};
use bevy::{prelude::*, input::mouse::MouseWheel};

pub fn spawn(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(LookCamera);
}

pub fn zoom(
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut OrthographicProjection), With<LookCamera>>,
) {
    for mouse_wheel in mouse_wheel_reader.iter() {
        let (_, mut projection) = query.single_mut();
        let zoom_scalar = 1.0 - constants::ZOOM_SENSITIVITY * mouse_wheel.y;
        let zoomed = projection.scale * zoom_scalar;
        projection.scale = zoomed.max(constants::ZOON_MIN).min(constants::ZOON_MAX);
    }
}
