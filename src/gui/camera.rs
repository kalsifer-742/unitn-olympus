use macroquad::{camera::{Camera3D}, experimental::camera::mouse, math::{vec2, vec3, Vec2, Vec3}, miniquad::gl::GL_SRC_ALPHA};

pub struct Camera {
    actual_camera: Camera3D,
    position: Vec3,
    target: Vec3,
    up: Vec3,
    front: Vec3,
    move_speed: f32,
    look_speed: f32,
    pitch: f32,
    yaw: f32,
    mouse_position: Vec2,
}

pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down
}

impl Camera {
    pub fn new(
        camera: Camera3D,
        front: Vec3,
        move_speed: f32,
        look_speed: f32,
        pitch: f32,
        yaw: f32,
        mouse_position: Vec2
    ) -> Self {
        let position = camera.position;
        let target = camera.target;
        let up = camera.up;
        
        Self {
            actual_camera: camera,
            position,
            target,
            up,
            front,
            move_speed,
            look_speed,
            pitch,
            yaw,
            mouse_position,
        }
    }

    pub fn update_position(&mut self, direction: Direction) {
        let front = self.front * self.move_speed;
        let right = self.front.cross(self.up).normalize() * self.move_speed;
    
        match direction {
            Direction::Forward => { self.position += front },
            Direction::Backward => { self.position -= front },
            Direction::Left => { self.position -= right },
            Direction::Right => { self.position += right },
            Direction::Up => { self.position.y += self.move_speed },
            Direction::Down => { self.position.y -= self.move_speed },
        }
    }

    pub fn update_orientation(&mut self, new_mouse_position: Vec2) {
        let mouse_delta = new_mouse_position - self.mouse_position;
        self.mouse_position = new_mouse_position;

        self.pitch += mouse_delta.y * -self.look_speed;
        self.yaw += mouse_delta.x * self.look_speed;
    
        self.pitch = self.pitch.clamp(-1.5, 1.5);
    }

    fn update(&mut self) {
        self.front = Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();
    
        //let right = front.cross(self.up).normalize();
        //self.up = Vec3::new(0,1,0);//right.cross(front).normalize();

        self.actual_camera.position = self.position;
        //self.actual_camera.up = self.up;
        self.actual_camera.target = self.position + self.front;
    }

    pub fn get_actual_camera(&mut self) -> &Camera3D {
        self.update();
        &self.actual_camera
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            actual_camera: Camera3D {
                position: vec3(-12.0, 10.0, 0.0),
                target: vec3(0.0, 0.0, 0.0),
                up: vec3(0.0, 1.0, 0.0),
                ..Default::default()
            },
            position: Vec3::default(),
            target: Vec3::default(),
            up: vec3(0.0, 1.0, 0.0),
            front: Vec3::default(),
            move_speed: 1.0,
            look_speed: 0.001,
            pitch: 0.0,
            yaw: 0.0,
            mouse_position: Vec2::default(),
        }
    }
}