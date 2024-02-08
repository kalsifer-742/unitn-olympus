use macroquad::prelude::*;
use robotics_lib::world::tile::{Tile, TileType};

pub struct Renderer;

pub struct RendererProps {
    pub world_map: Vec<Vec<Option<Tile>>>,
    pub world_map_size: usize
}

impl Default for RendererProps {
    fn default() -> Self {
        Self {
            world_map: vec![vec![None]],
            world_map_size: 0
        }
    }
}

impl Renderer {
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

    pub fn render(props: &RendererProps) {
        Renderer::draw_grid(props.world_map_size as u32, 1.0, BLACK, GRAY);

        for (x, row) in props.world_map.iter().enumerate() {
            for (z, tile) in row.iter().enumerate() {
                if let Some(tile) = tile {
                    let color = match tile.tile_type {
                        TileType::DeepWater => Color::from_hex(0x000066),
                        TileType::ShallowWater => Color::from_hex(0x3333ff),
                        TileType::Sand => Color::from_hex(0xffcc00),
                        TileType::Grass => Color::from_hex(0x009933),
                        TileType::Street => Color::from_hex(0x404040),
                        TileType::Hill => Color::from_hex(0x669900),
                        TileType::Mountain => Color::from_hex(0x996633),
                        TileType::Snow => Color::from_hex(0xccffff),
                        TileType::Lava => Color::from_hex(0xcf1020),
                        TileType::Teleport(_) => Color::from_hex(0x66ffff),
                        TileType::Wall => Color::from_hex(0x000000),
                    };

                    draw_affine_parallelepiped(
                        vec3(x as f32, 0.0, z as f32), //x as f32 * Vec3::X + z as f32 * Vec3::Z,
                        1.0 * Vec3::X,
                        (1.0 + tile.elevation as f32) * Vec3::Y,
                        1.0 * Vec3::Z,
                        None,
                        color
                    );
                    draw_cube_wires(
                        vec3(0.5 + x as f32, 0.5, 0.5 + z as f32),
                        vec3(1.0, 1.0, 1.0),
                        ORANGE
                    );
                }
            }
        }
    }
}