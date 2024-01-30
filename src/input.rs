use macroquad::input::{is_key_down, KeyCode};
use crate::camera::CameraConfig;

pub fn handle_keys(camera_config: &mut CameraConfig) {
    if is_key_down(KeyCode::W) {
        camera_config.position.x += 0.1;
    }
    if is_key_down(KeyCode::S) {
        camera_config.position.x -= 0.1;
    }
    if is_key_down(KeyCode::A) {
        camera_config.position.z -= 0.1;
    }
    if is_key_down(KeyCode::D) {
        camera_config.position.z += 0.1;
    }
    if is_key_down(KeyCode::Space) {
        camera_config.position.y += 0.1;
    }
    if is_key_down(KeyCode::LeftShift) {
        camera_config.position.y -= 0.1;
    }
}