use super::events::*;
use bevy::{prelude::*, render::camera::Camera};
use std::f32::EPSILON;

const FOUNDATION_POINT: Vec3 = Vec3::ZERO;
const FOUNDATION_NORMAL: Vec3 = Vec3::Y;

pub struct LookCamera;

pub struct MouseSettings {
    pub sensitivity: f32,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self { sensitivity: 0.01 }
    }
}

// Source: https://github.com/aevyrie/bevy_mod_raycast/blob/master/src/primitives.rs#L105
fn screen_to_world(
    windows: &Res<Windows>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor_pos: Vec2,
) -> Option<Vec3> {
    let window = windows.get_primary().unwrap();
    let screen_size = Vec2::from([window.width(), window.height()]);
    let cursor_ndc = (cursor_pos / screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
    let ndc_to_world: Mat4 = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
    let cursor_pos_near = ndc_to_world.project_point3(cursor_ndc.extend(-1.0));
    let cursor_pos_far = ndc_to_world.project_point3(cursor_ndc.extend(1.0));
    let ray_direction = cursor_pos_far - cursor_pos_near;
    let d = ray_direction.dot(FOUNDATION_NORMAL);

    if d.abs() <= EPSILON {
        None
    } else {
        let difference = cursor_pos_near - FOUNDATION_POINT;
        let p = difference.dot(FOUNDATION_NORMAL);
        let distance = p / d;
        Some(cursor_pos_near - ray_direction * distance)
    }
}

pub fn input_to_look(
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform), With<LookCamera>>,
    mut cursor_reader: EventReader<CursorMoved>,
    // mut settings: ResMut<MouseSettings>, TODO: Sensitivity integration
    mut look_writer: EventWriter<LookEvent>,
) {
    for cursor in cursor_reader.iter() {
        if let Ok((camera, camera_transform)) = query.single() {
            if let Some(world_cursor_position) =
                screen_to_world(&windows, camera, camera_transform, cursor.position)
            {
                look_writer.send(LookEvent::new(&world_cursor_position));
            }
        }
    }
}
