use macroquad::prelude::*;
use robotics_lib::world::tile::Tile;
use crate::gui::camera::Direction;

pub mod game_logic;
mod camera;

pub struct WorldMap {
    pub map_size: u64,
    pub explored_map: Vec<Vec<Option<Tile>>>,
}

impl WorldMap {
    pub fn new(map_size: u64, explored_map: Vec<Vec<Option<Tile>>>) -> Self {
        Self {
            map_size,
            explored_map,
        }
    }
}

impl Default for WorldMap {
    fn default() -> Self {
        Self { map_size: 0, explored_map: vec![vec![None]] }
    }
}

pub struct GUI {
    pub world_map: WorldMap,
    pub camera: camera::Camera,
}

impl GUI {
    pub fn new(world_map: WorldMap, camera: camera::Camera) -> Self {
        Self {
            world_map,
            camera,
        }
    }

    pub fn init(&self) {
        set_cursor_grab(true);
        show_mouse(false);
    }

    pub fn offering_to_the_gods(&mut self, explored_world: Vec<Vec<Option<Tile>>>) {
        self.world_map.explored_map = explored_world;
    }

    fn handle_keys(&mut self) {
        if is_key_down(KeyCode::W) {
            self.camera.update_position(Direction::Forward);
        }
        if is_key_down(KeyCode::S) {
            self.camera.update_position(Direction::Backward);
        }
        if is_key_down(KeyCode::A) {
            self.camera.update_position(Direction::Left);
        }
        if is_key_down(KeyCode::D) {
            self.camera.update_position(Direction::Right);
        }
        if is_key_down(KeyCode::Space) {
            self.camera.update_position(Direction::Up);
        }
        if is_key_down(KeyCode::LeftShift) {
            self.camera.update_position(Direction::Down);
        }
    }

    fn handle_mouse(&mut self) {
        self.camera.update_orientation(mouse_position().into())
    }

    pub fn handle_input(&mut self) {
        self.handle_keys();
        self.handle_mouse();
    }

    pub fn draw_background() {
        clear_background(LIGHTGRAY);
        draw_grid(20, 1., BLACK, GRAY);
    }

    pub fn draw_world(&self) {
        for (x, row) in self.world_map.explored_map.iter().enumerate() {
            for (z, tile) in row.iter().enumerate() {
                if let Some(tile) = tile {
                    draw_affine_parallelepiped(x as f32 * Vec3::X + z as f32 * Vec3::Z, 1.0 * Vec3::X, tile.elevation as f32 * Vec3::Y, 1.0 * Vec3::Z, None, BLACK);  
                }
            }
        }
    }
}

impl Default for GUI {
    fn default() -> Self {
        Self { 
            world_map: Default::default(),
            camera: Default::default(),
        }
    }
}

// pub trait Believer: Runnable {
//     fn offering_to_the_gods();
// }