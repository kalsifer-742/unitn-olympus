use macroquad::{camera::Camera3D, input::{is_key_down, KeyCode}};

pub fn handle_keys(camera: &mut Camera3D) {
    if is_key_down(KeyCode::W) {
        camera.position.x += 1.0;
    }
    if is_key_down(KeyCode::S) {
        camera.position.x -= 1.0;
    }
    if is_key_down(KeyCode::A) {
        camera.position.z -= 1.0;
    }
    if is_key_down(KeyCode::D) {
        camera.position.z += 1.0;
    }
    if is_key_down(KeyCode::Space) {
        camera.position.y += 1.0;
    }
    if is_key_down(KeyCode::LeftShift) {
        camera.position.y -= 1.0;
    }
}