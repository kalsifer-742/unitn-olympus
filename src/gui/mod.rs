use macroquad::prelude::*;
use robotics_lib::world::tile::Tile;

pub mod game_logic;
mod input;

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
    pub camera: Camera3D,
}

impl GUI {
    pub fn new(world_map: WorldMap, camera: Camera3D) -> Self {
        Self {
            world_map,
            camera,
        }
    }

    pub fn offering_to_the_gods(&mut self, explored_world: Vec<Vec<Option<Tile>>>) {
        self.world_map.explored_map = explored_world;
    }

    pub fn handle_input(&mut self) {
        input::handle_keys(&mut self.camera);
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
            camera: Camera3D {
                position: vec3(-12.0, 10.0, 0.0),
                target: vec3(0.0, 0.0, 0.0),
                up: vec3(0.0, 1.0, 0.0),
                ..Default::default()
            },
        }
    }
}

// pub trait Believer: Runnable {
//     fn offering_to_the_gods();
// }