use crate::lib::events::*;
use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct MouseSettings {
    pub sensitivity: f32,
    pub position: Vec3,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.01,
            position: Vec3::ZERO,
        }
    }
}

pub fn input_to_look(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut settings: ResMut<MouseSettings>,
    mut look_events: EventWriter<LookEvent>,
) {
    let mut look = Vec2::ZERO;
    for motion in mouse_motion_events.iter() {
        look += motion.delta;
    }
    if look.length_squared() > 1E-6 {
        look *= settings.sensitivity;
        settings.position += look.extend(0.0);
        look_events.send(LookEvent::new(&settings.position));
    }
}
