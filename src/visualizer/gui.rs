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
    show_stats: bool,
    quit_requested: bool,
    exit: bool,
    is_mouse_grabbed: bool,
}

pub struct GUIProps {
    pub discoverable_tiles: usize,
    pub robot_coordinates: (usize, usize),
    pub robot_energy: usize,
    pub robot_backpack_contents: HashMap<Content, usize>,
    pub robot_backpack_size: usize,
    pub robot_score: f32,
}

impl Default for GUIProps {
    fn default() -> Self {
        Self {
            discoverable_tiles: usize::MAX,
            robot_score: 0.0,
            robot_energy: 0,
            robot_coordinates: (0, 0),
            robot_backpack_contents: HashMap::default(),
            robot_backpack_size: 0,
        }
    }
}

impl GUI {
    pub(crate) fn grab_mouse(&mut self, grabbed: bool) {
        if grabbed {
            set_cursor_grab(true);
            show_mouse(false);
        } else {
            set_cursor_grab(false);
            show_mouse(true);
        }
    }

    fn toggle_mouse_grab(&mut self) {
        self.is_mouse_grabbed = !self.is_mouse_grabbed;
        self.grab_mouse(self.is_mouse_grabbed);
    }

    pub(crate) fn handle_input(&mut self) {
        if is_key_pressed(self.keyboard_controls.exit){
            self.quit_requested = true;
        }
        if is_key_pressed(self.keyboard_controls.toggle_free_mouse) {
            self.toggle_mouse_grab();
        }
        if is_key_pressed(self.keyboard_controls.toggle_help) {
            self.show_help = !self.show_help;
        }
        if is_key_pressed(self.keyboard_controls.toggle_statistics) {
            self.show_stats = !self.show_stats;
        }
    }

    fn show_exit_dialog(&mut self) {
        let dialog_position = vec2(
            self.viewport_width / 2.0 - self.window_size / 4.0,
            self.viewport_height / 2.0 - self.window_size / 6.0
        );
        widgets::Window::new(
            hash!("exit_dialog"),
            dialog_position,
            vec2(self.window_size / 2.0, self.window_size / 4.0)
        )
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Do you really want to quit?");
            ui.separator();
            //ui.same_line(60.0);
            if ui.button(None, "Yes") {
                self.exit = true;
            }
            //ui.same_line(120.);
            if ui.button(None, "No") {
                self.grab_mouse(self.is_mouse_grabbed);
                self.quit_requested = false;
            }
        });
    }

    pub fn exit(&self) -> bool {
        self.exit
    }

    fn map_range(x: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> f32 {
        (x - x_min) * ((y_max - y_min) / (x_max - x_min)) + y_min
    }

    fn show_robot_info(&self, props: &GUIProps) {
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
                    Self::map_range(props.robot_energy as f32, 0., 1000., 0., 100.),
                    20.0),
                Color::new(0.0, 0.0, 0.0, 1.0),
                YELLOW,
            );

            ui.separator();

            ui.label(None, &format!("Coordinates X: {}, Y: {}", props.robot_coordinates.0, props.robot_coordinates.1));
            
            ui.separator();

            widgets::Group::new(hash!("backpack"), vec2(350., 200.))
            .ui(ui, |ui| {
                for (index, (item, amount)) in props.robot_backpack_contents.iter().enumerate() {
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

            ui.label(None, &format!("Backpack size: {}", props.robot_backpack_size));
            ui.label(None, &format!("Discoverable tiles: {}", props.discoverable_tiles));
            ui.label(None, &format!("Score: {}", props.robot_score));
        });
    }

    fn show_stats(&self) {
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

    fn show_help(&self) {
        widgets::Window::new(
            hash!("help"), 
            vec2(0.0, self.viewport_height - self.window_size),
            vec2(self.window_size, self.window_size)
        )
        .label("Help")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("Toggle mouse grab: J"));
            ui.label(None, &format!("Toggle statistics window: F3"));
        });
    }

    pub(crate) fn show(&mut self, props: &GUIProps) {
        draw_text("Press H for help", 0.0, self.viewport_height, 30.0, GREEN);

        self.show_robot_info(props);
        
        if self.quit_requested {
            self.grab_mouse(false);
            self.show_exit_dialog();
        }
        if self.show_stats {
            self.show_stats();
        }
        if self.show_help {
            self.show_help();
        }
    }
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            viewport_width: screen_width(),
            viewport_height: screen_height(),
            window_size: 400.0,
            keyboard_controls: Default::default(),
            show_help: false,
            show_stats: false,
            quit_requested: false,
            exit: false,
            is_mouse_grabbed: true,
        }
    }
}