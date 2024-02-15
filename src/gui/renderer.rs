use macroquad::prelude::*;
use robotics_lib::world::{environmental_conditions::DayTime, tile::{Content, Tile, TileType}};

struct Textures {
    robot: Texture2D,
    day_sky: Texture2D,
    afternoon_sky: Texture2D,
    night_sky: Texture2D,
    water_block: Texture2D,
    sand_block: Texture2D,
    grass_block: Texture2D,
    street_block: Texture2D,
    hill_block: Texture2D,
    mountain_block: Texture2D,
    snow_block: Texture2D,
    lava_block: Texture2D,
    teleport_block: Texture2D,
    wall_block: Texture2D,
    rock_content: Texture2D,
    tree_content: Texture2D,
    garbage_content: Texture2D,
    fire_content: Texture2D,
    coin_content: Texture2D,
    bin_content: Texture2D,
    crate_content: Texture2D,
    bank_content: Texture2D,
    water_content: Texture2D,
    market_content: Texture2D,
    fish_content: Texture2D,
    building_content: Texture2D,
    bush_content: Texture2D,
    jolly_block_content: Texture2D,
    scarecrow_content: Texture2D
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
        self.rock_content.set_filter(FilterMode::Nearest);
        self.tree_content.set_filter(FilterMode::Nearest);
        self.garbage_content.set_filter(FilterMode::Nearest);
        self.fire_content.set_filter(FilterMode::Nearest);
        self.coin_content.set_filter(FilterMode::Nearest);
        self.bin_content.set_filter(FilterMode::Nearest);
        self.crate_content.set_filter(FilterMode::Nearest);
        self.bank_content.set_filter(FilterMode::Nearest);
        self.water_content.set_filter(FilterMode::Nearest);
        self.market_content.set_filter(FilterMode::Nearest);
        self.fish_content.set_filter(FilterMode::Nearest);
        self.building_content.set_filter(FilterMode::Nearest);
        self.bush_content.set_filter(FilterMode::Nearest);
        self.jolly_block_content.set_filter(FilterMode::Nearest);
        self.scarecrow_content.set_filter(FilterMode::Nearest);
    }
}

impl Default for Textures {
    fn default() -> Self {
        Self {
            robot: Texture2D::from_file_with_format(include_bytes!("../../assets/robot/creeper.png"), Some(ImageFormat::Png)),
            day_sky: Texture2D::from_file_with_format(include_bytes!("../../assets/skybox/day_sky.png"), Some(ImageFormat::Png)),
            afternoon_sky: Texture2D::from_file_with_format(include_bytes!("../../assets/skybox/afternoon_sky.png"), Some(ImageFormat::Png)),
            night_sky: Texture2D::from_file_with_format(include_bytes!("../../assets/skybox/night_sky.png"), Some(ImageFormat::Png)),
            water_block: Texture2D::from_file_with_format(include_bytes!("../../assets/tiles/underwater_opaque.png"), Some(ImageFormat::Png)),
            sand_block: Texture2D::from_file_with_format(include_bytes!("../../assets/tiles/sand.png"), Some(ImageFormat::Png)),
            grass_block: Texture2D::from_file_with_format(include_bytes!("../../assets/tiles/green_concrete_powder.png"),Some(ImageFormat::Png)),
            street_block: Texture2D::from_file_with_format(include_bytes!("../../assets/tiles/dirt_path_top.png"), Some(ImageFormat::Png)),
            hill_block: Texture2D::from_file_with_format(include_bytes!("../../assets/tiles/dirt.png"), Some(ImageFormat::Png)),
            mountain_block: Texture2D::from_file_with_format(include_bytes!("../../assets/tiles/stone.png"), Some(ImageFormat::Png)),
            snow_block: Texture2D::from_file_with_format(include_bytes!("../../assets/tiles/snow.png"), Some(ImageFormat::Png)),
            lava_block: Texture2D::from_file_with_format(include_bytes!("../../assets/tiles/lava.png"), Some(ImageFormat::Png)),
            teleport_block: Texture2D::from_file_with_format(include_bytes!("../../assets/tiles/beacon.png"), Some(ImageFormat::Png)),
            wall_block: Texture2D::from_file_with_format(include_bytes!("../../assets/tiles/stone_bricks.png"), Some(ImageFormat::Png)),
            rock_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/cobblestone.png"), Some(ImageFormat::Png)),
            tree_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/oak_log.png"), Some(ImageFormat::Png)),
            garbage_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/coal_block.png"), Some(ImageFormat::Png)),
            fire_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/fire.png"), Some(ImageFormat::Png)),
            coin_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/gold_ore.png"), Some(ImageFormat::Png)),
            bin_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/dropper_front_vertical.png"), Some(ImageFormat::Png)),
            crate_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/barrel.png"), Some(ImageFormat::Png)),
            bank_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/gold_block.png"), Some(ImageFormat::Png)),
            water_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/water.png"), Some(ImageFormat::Png)),
            market_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/emerald_block.png"), Some(ImageFormat::Png)),
            fish_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/orange_glazed_terracotta.png"), Some(ImageFormat::Png)),
            building_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/bricks.png"), Some(ImageFormat::Png)),
            bush_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/azalea_top.png"), Some(ImageFormat::Png)),
            jolly_block_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/jack_o_lantern.png"), Some(ImageFormat::Png)),
            scarecrow_content: Texture2D::from_file_with_format(include_bytes!("../../assets/contents/hay_block_side.png"), Some(ImageFormat::Png)),
        }
    }
}

#[derive(Clone)]
pub(super) struct RendererProps<'a> {
    pub explored_world_map: &'a Vec<Vec<Option<Tile>>>,
    pub robot_coordinates: (usize, usize),
    pub time_of_day: DayTime
}

pub(super) struct Renderer {
    world_map_size: usize,
    textures: Textures,
}

impl Renderer {
    pub(super) fn new(world_map_size: usize) -> Self {
        let textures = Textures::default();
        textures.init();
            
        Self {
            world_map_size,
            textures,
        }
    }

    fn draw_background(&self, props: &RendererProps, daylight_cycle: bool) {
        clear_background(SKYBLUE);

        if daylight_cycle {
            let texture = match props.time_of_day {
                DayTime::Morning => &self.textures.day_sky,
                DayTime::Afternoon => &self.textures.afternoon_sky,
                DayTime::Night => &self.textures.night_sky,
            };
    
            draw_sphere(
                vec3(self.world_map_size as f32 / 2.0, 0.0, self.world_map_size as f32 / 2.0), 
                self.world_map_size as f32 * 2.0, 
                Some(texture),
                WHITE,
            );
        }
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
        let offset = 0.5;

        for (x, row) in props.explored_world_map.iter().enumerate() {
            for (z, tile) in row.iter().enumerate() {
                if let Some(tile) = tile {
                    let mut color = WHITE;
                    let tile_texture = match tile.tile_type {
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

                    let content_texture = match tile.content {
                        Content::Rock(_) => &self.textures.rock_content,
                        Content::Tree(_) => &self.textures.tree_content,
                        Content::Garbage(_) => &self.textures.garbage_content,
                        Content::Fire => &self.textures.fire_content,
                        Content::Coin(_) => &self.textures.coin_content,
                        Content::Bin(_) => &self.textures.bin_content,
                        Content::Crate(_) => &self.textures.crate_content,
                        Content::Bank(_) => &self.textures.bank_content,
                        Content::Water(_) => &self.textures.water_content,
                        Content::Market(_) => &self.textures.market_content,
                        Content::Fish(_) => &self.textures.fish_content,
                        Content::Building => &self.textures.building_content,
                        Content::Bush(_) => &self.textures.bush_content,
                        Content::JollyBlock(_) => &self.textures.jolly_block_content,
                        Content::Scarecrow => &self.textures.scarecrow_content,
                        Content::None => tile_texture,
                    };
                    
                    draw_affine_parallelepiped(
                        vec3(x as f32, 0.0, z as f32), //x as f32 * Vec3::X + z as f32 * Vec3::Z,
                        1.0 * Vec3::X,
                        (tile.elevation as f32) * Vec3::Y,
                        1.0 * Vec3::Z,
                        Some(tile_texture),
                        color
                    );

                    match tile.content {
                        Content::None => {}
                        Content::Water(_) => {
                            draw_cube_wires(
                                vec3(offset + x as f32, tile.elevation as f32, offset + z as f32),
                                vec3(0.5, 0.5, 0.5),
                                BLUE
                            );
                        }
                        _ => {
                            draw_cube(
                                vec3(offset + x as f32, 0.25 + tile.elevation as f32, offset + z as f32),
                                vec3(0.5, 0.5, 0.5),
                                Some(content_texture),
                                WHITE
                            );
                        }
                    }
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

    pub(super) fn render(&self, props: RendererProps, daylight_cycle: bool) {       
        self.draw_background(&props, daylight_cycle);
        self.draw_grid(1.0, BLACK, DARKGRAY);
        self.render_explored_map(&props);
        self.render_robot(&props);
    }
}