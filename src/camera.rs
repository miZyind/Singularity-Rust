use crate::{constants, lib::look::LookCamera};
use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    render::camera::{Projection, ScalingMode},
};

pub fn spawn(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            projection: OrthographicProjection {
                scale: 5.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(LookCamera);
}

pub fn zoom(
    mut mouse_wheel_reader: EventReader<MouseWheel>,
    mut query: Query<&mut Projection, With<Camera>>,
) {
    for mouse_wheel in mouse_wheel_reader.iter() {
        if let Ok(mut projection) = query.get_single_mut() {
            if let Projection::Orthographic(orthographic) = &*projection {
                let zoom_scalar = 1.0 - constants::ZOOM_SENSITIVITY * mouse_wheel.y;
                let zoomed = orthographic.scale * zoom_scalar;
                let scale = zoomed.max(constants::ZOON_MIN).min(constants::ZOON_MAX);
                *projection = Projection::Orthographic(OrthographicProjection {
                    scale,
                    scaling_mode: ScalingMode::FixedVertical(2.0),
                    ..default()
                });
            }
        }
    }
}
