use std::vec;

use macroquad::prelude::*;
use robotics_lib::world::tile::{Tile, TileType};

struct Textures {
    robot: Texture2D,
    sky: Texture2D,
    water_block: Texture2D,
    sand_block: Texture2D,
    grass_block: Texture2D,
    street_block: Texture2D,
    hill_block: Texture2D,
    mountain_block: Texture2D,
    snow_block: Texture2D,
    lava_block: Texture2D,
    teleport_block: Texture2D,
    wall_block: Texture2D
}

impl Textures {
    fn init(&self) {
        self.robot.set_filter(FilterMode::Nearest);
        self.water_block.set_filter(FilterMode::Nearest);
        self.sand_block.set_filter(FilterMode::Nearest);
        self.grass_block.set_filter(FilterMode::Nearest);
        self.street_block.set_filter(FilterMode::Nearest);
        self.hill_block.set_filter(FilterMode::Nearest);
        self.mountain_block.set_filter(FilterMode::Nearest);
        self.snow_block.set_filter(FilterMode::Nearest);
        self.lava_block.set_filter(FilterMode::Nearest);
        self.teleport_block.set_filter(FilterMode::Nearest);
        self.wall_block.set_filter(FilterMode::Nearest);
    }
}

impl Default for Textures {
    fn default() -> Self {
        Self {
            robot: Texture2D::from_file_with_format(include_bytes!("../../assets/creeper.png"), Some(ImageFormat::Png)),
            sky: Texture2D::from_file_with_format(include_bytes!("../../assets/day_sky.png"), Some(ImageFormat::Png)),
            water_block: Texture2D::from_file_with_format(include_bytes!("../../assets/underwater_opaque.png"), Some(ImageFormat::Png)),
            sand_block: Texture2D::from_file_with_format(include_bytes!("../../assets/sand.png"), Some(ImageFormat::Png)),
            grass_block: Texture2D::from_file_with_format(include_bytes!("../../assets/green_concrete_powder.png"),Some(ImageFormat::Png)),
            street_block: Texture2D::from_file_with_format(include_bytes!("../../assets/dirt_path_top.png"), Some(ImageFormat::Png)),
            hill_block: Texture2D::from_file_with_format(include_bytes!("../../assets/dirt.png"), Some(ImageFormat::Png)),
            mountain_block: Texture2D::from_file_with_format(include_bytes!("../../assets/stone.png"), Some(ImageFormat::Png)),
            snow_block: Texture2D::from_file_with_format(include_bytes!("../../assets/snow.png"), Some(ImageFormat::Png)),
            lava_block: Texture2D::from_file_with_format(include_bytes!("../../assets/lava.png"), Some(ImageFormat::Png)),
            teleport_block: Texture2D::from_file_with_format(include_bytes!("../../assets/end_portal.png"), Some(ImageFormat::Png)),
            wall_block: Texture2D::from_file_with_format(include_bytes!("../../assets/cobblestone.png"), Some(ImageFormat::Png)),
        }
    }
}

pub struct Renderer {
    world_map_size: usize,
    textures: Textures
}

pub struct RendererProps {
    pub explored_world_map: Vec<Vec<Option<Tile>>>,
    pub robot_coordinates: (usize, usize),
}

impl Default for RendererProps {
    fn default() -> Self {
        Self {
            explored_world_map: vec![vec![None]],
            robot_coordinates: (0, 0)
        }
    }
}

impl Renderer {
    pub fn new(world_map_size: usize) -> Self {        
        let textures = Textures::default();
        textures.init();
        
        Self {
            world_map_size,
            textures
        }
    }

    pub fn draw_background(&self) {
        clear_background(LIGHTGRAY);

        draw_sphere(
            vec3(self.world_map_size as f32 / 2.0, 0.0, self.world_map_size as f32 / 2.0), 
            self.world_map_size as f32 * 2.0, 
            Some(&self.textures.sky),
            WHITE,
        );
    }

    fn draw_grid(&self, spacing: f32, axes_color: Color, other_color: Color) {
        let slices = self.world_map_size as u32;
        
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

    fn render_explored_map(&self, props: &RendererProps) {                
        for (x, row) in props.explored_world_map.iter().enumerate() {
            for (z, tile) in row.iter().enumerate() {
                if let Some(tile) = tile {
                    let mut color = WHITE;
                    let texture = match tile.tile_type {
                        TileType::DeepWater => { color = GRAY; &self.textures.water_block }
                        TileType::ShallowWater => &self.textures.water_block,
                        TileType::Sand => &self.textures.sand_block,
                        TileType::Grass => &self.textures.grass_block,
                        TileType::Street => &self.textures.street_block,
                        TileType::Hill => &self.textures.hill_block,
                        TileType::Mountain => &self.textures.mountain_block,
                        TileType::Snow => &self.textures.snow_block,
                        TileType::Lava => &self.textures.lava_block,
                        TileType::Teleport(_) => &self.textures.teleport_block,
                        TileType::Wall => &self.textures.wall_block,
                    };
                    
                    draw_affine_parallelepiped(
                        vec3(x as f32, 0.0, z as f32), //x as f32 * Vec3::X + z as f32 * Vec3::Z,
                        1.0 * Vec3::X,
                        (tile.elevation as f32) * Vec3::Y,
                        1.0 * Vec3::Z,
                        Some(texture),
                        color
                    );
                }
            }
        }
    }

    fn render_robot(&self, props: &RendererProps) {
        let offset = 0.5;
        let (x, z) = props.robot_coordinates;
        
        if let Some(tile) = &props.explored_world_map[x][z] {
            draw_line_3d(
                vec3(offset + x as f32, self.world_map_size as f32, offset + z as f32),
                vec3(offset + x as f32, tile.elevation as f32, offset + z as f32),
                GREEN
            );
            draw_cube(
                vec3(offset + x as f32, offset + tile.elevation as f32, offset + z as f32),
                vec3(1.0, 1.0, 1.0),
                Some(&self.textures.robot),
                WHITE
            );
        }
    }

    pub fn render(&self, props: &RendererProps) {
        self.draw_background();
        self.draw_grid(1.0, BLACK, GRAY);
        self.render_explored_map(props);
        self.render_robot(props);
    }
}