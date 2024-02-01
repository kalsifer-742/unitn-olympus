use macroquad::prelude::*;
use robotics_lib::world::tile::{Tile, TileType};
use crate::gui::camera::Direction;

pub mod game_logic;
mod camera;

pub struct WorldMap {
    pub map_size: usize,
    pub explored_map: Vec<Vec<Option<Tile>>>,
}

impl WorldMap {
    pub fn new(map_size: usize, explored_map: Vec<Vec<Option<Tile>>>) -> Self {
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

    pub fn init(&mut self, map_size: usize) {
        set_cursor_grab(true);
        show_mouse(false);
        self.world_map.map_size = map_size;
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
    }

    fn draw_grid(slices: u32, spacing: f32, axes_color: Color, other_color: Color) {
        for i in 0..slices + 1 {
            let color = if i == 0 { axes_color } else { other_color };
            
            //horizontal lines
            draw_line_3d(
                vec3(i as f32 * spacing, 0.0, 0.0),
                vec3(i as f32 * spacing, 0., slices as f32 * spacing),
                color,
            );
            //vertical lines
            draw_line_3d(
                vec3(0.0, 0.0, i as f32 * spacing),
                vec3(slices as f32 * spacing, 0., i as f32 * spacing),
                color,
            );
        }
    }

    fn draw_pillar(&self, coordinate: (usize, usize), tile: &Tile, texture: &Texture2D){
        for level in 0..tile.elevation {
            match tile.tile_type {
                TileType::DeepWater => {
                    draw_cube(
                        vec3(coordinate.0 as f32, 0.5 + level as f32, coordinate.1 as f32),
                        vec3(1.0, 1.0, 1.0),
                        Some(
                            texture
                        ),
                        LIGHTGRAY
                    );
                },
                TileType::ShallowWater => {
                    draw_cube(
                        vec3(coordinate.0 as f32, 0.5 + level as f32, coordinate.1 as f32),
                        vec3(1.0, 1.0, 1.0),
                        Some(
                            texture
                        ),
                        WHITE
                    );
                },
                _ => {
                    draw_cube(
                        vec3(coordinate.0 as f32, 0.5 + level as f32, coordinate.1 as f32),
                        vec3(1.0, 1.0, 1.0),
                        Some(
                            texture
                        ),
                        WHITE
                    );
                }
            }
        }
    }

    pub fn draw_world(&self) {
        let water_block = Texture2D::from_file_with_format(
            include_bytes!("../../assets/underwater_opaque.png"), 
            Some(ImageFormat::Png)
        );
        let sand_block = Texture2D::from_file_with_format(
            include_bytes!("../../assets/sand.png"), 
            Some(ImageFormat::Png)
        );
        let grass_block = Texture2D::from_file_with_format(
            include_bytes!("../../assets/green_concrete_powder.png"),
            Some(ImageFormat::Png)
        );
        let street_block = Texture2D::from_file_with_format(
            include_bytes!("../../assets/dirt_path_top.png"), 
            Some(ImageFormat::Png)
        );
        let hill_block = Texture2D::from_file_with_format(
            include_bytes!("../../assets/dirt.png"), 
            Some(ImageFormat::Png)
        );
        let mountain_block = Texture2D::from_file_with_format(
            include_bytes!("../../assets/stone.png"), 
            Some(ImageFormat::Png)
        );
        let snow_block = Texture2D::from_file_with_format(
            include_bytes!("../../assets/snow.png"), 
            Some(ImageFormat::Png)
        );
        let lava_block = Texture2D::from_file_with_format(
            include_bytes!("../../assets/lava.png"),
            Some(ImageFormat::Png)
        );
        let teleport_block = Texture2D::from_file_with_format(
            include_bytes!("../../assets/end_portal.png"), 
            Some(ImageFormat::Png)
        );
        let wall_block = Texture2D::from_file_with_format(
            include_bytes!("../../assets/cobblestone.png"), 
            Some(ImageFormat::Png)
        );

        water_block.set_filter(FilterMode::Nearest);
        sand_block.set_filter(FilterMode::Nearest);
        grass_block.set_filter(FilterMode::Nearest);
        street_block.set_filter(FilterMode::Nearest);
        hill_block.set_filter(FilterMode::Nearest);
        mountain_block.set_filter(FilterMode::Nearest);
        snow_block.set_filter(FilterMode::Nearest);
        lava_block.set_filter(FilterMode::Nearest);
        teleport_block.set_filter(FilterMode::Nearest);
        wall_block.set_filter(FilterMode::Nearest);

        GUI::draw_grid(self.world_map.map_size as u32, 1.0, BLACK, GRAY);

        for (x, row) in self.world_map.explored_map.iter().enumerate() {
            for (z, tile) in row.iter().enumerate() {
                if let Some(tile) = tile {
                    let texture = match tile.tile_type {
                        TileType::DeepWater => &water_block,
                        TileType::ShallowWater => &water_block,
                        TileType::Sand => &sand_block,
                        TileType::Grass => &grass_block,
                        TileType::Street => &street_block,
                        TileType::Hill => &hill_block,
                        TileType::Mountain => &mountain_block,
                        TileType::Snow => &snow_block,
                        TileType::Lava => &lava_block,
                        TileType::Teleport(_) => &teleport_block,
                        TileType::Wall => &wall_block,
                    };

                    self.draw_pillar((x, z), tile, texture);
                    // draw_cube_wires(
                    //     vec3(x as f32, tile.elevation as f32, z as f32),
                    //     vec3(1.0, 1.0, 1.0),
                    //     ORANGE
                    // );
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