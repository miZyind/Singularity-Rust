use crate::{constants, lib::look::LookCamera};
use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
};

pub fn spawn(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 5.0;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn_bundle(camera).insert(LookCamera);
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn zoom(
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut OrthographicProjection), With<LookCamera>>,
) {
    for mouse_wheel in mouse_wheel_reader.iter() {
        if let Ok((_, mut projection)) = query.single_mut() {
            let zoom_scalar = 1.0 - constants::ZOOM_SENSITIVITY * mouse_wheel.y;
            let zoomed = projection.scale * zoom_scalar;
            projection.scale = zoomed.max(constants::ZOON_MIN).min(constants::ZOON_MAX);
        }
    }
}
