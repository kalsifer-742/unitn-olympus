use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::telemetry::textures_count;
use macroquad::ui::{root_ui, widgets};
use macroquad::hash;
use robotics_lib::world::tile::Content;
use crate::visualizer::controls::KeyboardControls;

pub struct GUI {
    viewport_width: f32,
    viewport_height: f32,
    window_size: f32,
    keyboard_controls: KeyboardControls,
    show_help: bool,
    show_stats: bool
}

pub struct GUIProps {
    pub energy: usize,
    pub coordinates: (usize, usize),
    pub backpack_contents: HashMap<Content, usize>,
    pub backpack_size: usize
}

impl Default for GUIProps {
    fn default() -> Self {
        Self {
            energy: 0,
            coordinates: (0, 0),
            backpack_contents: HashMap::default(),
            backpack_size: 0,
        }
    }
}

impl GUI {
    pub fn init(&mut self) {
        set_cursor_grab(true);
        show_mouse(false);
    }

    pub fn handle_input(&mut self) {
        if is_key_pressed(self.keyboard_controls.toggle_help) {
            self.show_help = !self.show_help;
        }
        if is_key_pressed(self.keyboard_controls.toggle_statistics) {
            self.show_stats = !self.show_stats;
        }
    }

    fn map_range(x: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> f32 {
        (x - x_min) * ((y_max - y_min) / (x_max - x_min)) + y_min
    }

    fn draw_robot_info(&self, props: &GUIProps) {
        widgets::Window::new(
            hash!("robot_info"),
            vec2(self.viewport_width - self.window_size, 0.),
            vec2(400., 400.)
        )
        .label("Robot")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Energy");
            
            let cursor = ui.canvas().cursor();
            ui.canvas().rect(
                Rect::new(cursor.x, cursor.y,
                    Self::map_range(props.energy as f32, 0., 1000., 0., 100.),
                    20.0),
                Color::new(0.0, 0.0, 0.0, 1.0),
                YELLOW,
            );

            ui.label(None, &format!("Coordinates X: {}, Y: {}", props.coordinates.0, props.coordinates.1));
            
            ui.separator();

            widgets::Group::new(hash!("backpack"), vec2(350., 200.))
            .ui(ui, |ui| {
                for (index, (item, amount)) in props.backpack_contents.iter().enumerate() {
                    widgets::Group::new(hash!("backpack_item", index), vec2(70., 70.))
                    .ui(ui, |ui| {
                        let item_name = match item {
                            Content::Rock(_) => "rock",
                            Content::Tree(_) => "tree",
                            Content::Garbage(_) => "garbage",
                            Content::Fire => "fire",
                            Content::Coin(_) => "coin",
                            Content::Bin(_) => "bin",
                            Content::Crate(_) => "crate",
                            Content::Bank(_) => "bank",
                            Content::Water(_) => "water",
                            Content::Market(_) => "market",
                            Content::Fish(_) => "fish",
                            Content::Building => "building",
                            Content::Bush(_) => "bush",
                            Content::JollyBlock(_) => "jollyblock",
                            Content::Scarecrow => "scarecrow",
                            Content::None => "none",
                        };
    
                        ui.label(None, &format!("{}", item_name));
                        ui.label(vec2(55., 50.), &format!("{}", amount));
                    });
                }
            });

            ui.separator();

            ui.label(None, &format!("Backpack size: {}", props.backpack_size));
        });
    }

    fn draw_stats(&self) {
        widgets::Window::new(
            hash!("stats"), 
            vec2(0.0, 0.0), 
            vec2(self.window_size, self.window_size)
        )
        .label("Statistics")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("FPS: {}", get_fps()));
            ui.label(None, &format!("Texture count: {}", textures_count()));
        });
    }

    fn draw_help(&self) {
        widgets::Window::new(
            hash!("help"), 
            vec2(0.0, self.viewport_height - self.window_size),
            vec2(self.window_size, self.window_size)
        )
        .label("Help")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("Toggle statistics window: F3"));
        });
    }

    pub fn draw(&self, props: &GUIProps) {
        self.draw_robot_info(props);
        
        if self.show_stats {
            self.draw_stats();
        }
        if self.show_help {
            self.draw_help();
        }
    }
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            viewport_width: 1920.0,
            viewport_height: 1080.0,
            window_size: 400.0,
            keyboard_controls: Default::default(),
            show_help: false,
            show_stats: false
        }
    }
}