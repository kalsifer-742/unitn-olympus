use macroquad::input::KeyCode;

pub struct KeyboardControls {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub toggle_help: KeyCode,
    pub toggle_statistics: KeyCode
}

impl Default for KeyboardControls {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            move_up: KeyCode::Space,
            move_down: KeyCode::LeftShift,
            toggle_help: KeyCode::H,
            toggle_statistics: KeyCode::F3,
        }
    }
}
