use macroquad::math::Vec3;

pub struct CameraConfig {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3
}

impl CameraConfig {
    pub fn new(position: Vec3, target: Vec3, up: Vec3) -> Self {
        Self { position, target, up }
    }
}