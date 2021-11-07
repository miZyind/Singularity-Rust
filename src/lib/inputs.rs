use bevy::input::keyboard::KeyCode;

pub struct Inputs {
    pub key_forward: KeyCode,
    pub key_backward: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_jump: KeyCode,
    pub key_run: KeyCode,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            key_forward: KeyCode::W,
            key_backward: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_jump: KeyCode::Space,
            key_run: KeyCode::LShift,
        }
    }
}
