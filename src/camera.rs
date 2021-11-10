use crate::{constants, MyRaycastSet};
use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::OrthographicProjection};
use bevy_mod_raycast::RayCastSource;

pub fn spawn(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_3d();

    camera.orthographic_projection.scale = 10.0;
    camera.transform = Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands
        .spawn_bundle(camera)
        .insert(RayCastSource::<MyRaycastSet>::new());
}

pub fn zoom(
    mut event_reader: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection>,
) {
    let mut zoom_scalar = 1.0;
    for event in event_reader.iter() {
        zoom_scalar *= 1.0 - constants::ZOOM_SENSITIVITY * event.y;
    }
    for mut projection in query.iter_mut() {
        let zoomed = projection.scale * zoom_scalar;
        projection.scale = zoomed.max(constants::ZOON_MIN).min(constants::ZOON_MAX);
    }
}
