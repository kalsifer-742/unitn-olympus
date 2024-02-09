use macroquad::prelude::*;
use robotics_lib::world::tile::{Tile, TileType};

pub struct Renderer {
    // world_map: Vec<Vec<Tile>>,
    world_map_size: usize,
}

pub struct RendererProps {
    pub explored_world_map: Vec<Vec<Option<Tile>>>,
}

impl Default for RendererProps {
    fn default() -> Self {
        Self {
            explored_world_map: vec![vec![None]],
        }
    }
}

impl Renderer {
    pub fn new(world_map_size: usize) -> Self {
        Self {
            // world_map,
            world_map_size
        }
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

    fn render_explored_map(props: &RendererProps) {
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
        
        for (x, row) in props.explored_world_map.iter().enumerate() {
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
    
                    draw_affine_parallelepiped(
                        vec3(x as f32, 0.0, z as f32), //x as f32 * Vec3::X + z as f32 * Vec3::Z,
                        1.0 * Vec3::X,
                        (1.0 + tile.elevation as f32) * Vec3::Y,
                        1.0 * Vec3::Z,
                        Some(texture),
                        WHITE
                    );
                }
            }
        }
    }

    pub fn render(&self, props: &RendererProps) {
        Self::draw_grid(self.world_map_size as u32, 1.0, BLACK, GRAY);
        Self::render_explored_map(props);
    }
}