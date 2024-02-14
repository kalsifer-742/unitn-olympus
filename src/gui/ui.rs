use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use macroquad::prelude::*;
use macroquad::telemetry::textures_count;
use macroquad::ui::{root_ui, widgets, Layout};
use macroquad::hash;
use robotics_lib::world::environmental_conditions::{DayTime, WeatherType};
use robotics_lib::world::tile::Content;
use crate::gui::keyboard_controls::KeyboardControls;

pub struct UI {
    viewport_width: f32,
    viewport_height: f32,
    keyboard_controls: KeyboardControls,
    show_help: bool,
    show_stats: bool,
    quit_requested: bool,
    exit: bool,
    is_mouse_grabbed: bool,
    mouse_grabbed_flag: bool,
    old_grab_status: bool,
    tick_time: Rc<RefCell<f32>>,
    daylight_cycle: bool,
}

pub struct UIProps<'a> {
    pub discoverable_tiles: usize,
    pub robot_coordinates: (usize, usize),
    pub robot_energy: usize,
    pub robot_backpack_contents: &'a HashMap<Content, usize>,
    pub robot_backpack_size: usize,
    pub robot_score: f32,
    pub time_of_day: DayTime,
    pub time_of_day_string: String,
    pub weather_condition: WeatherType,
}

impl UI {
    pub fn new(tick_time: Rc<RefCell<f32>>) -> Self {
        Self {
            viewport_width: screen_width(),
            viewport_height: screen_height(),
            keyboard_controls: Default::default(),
            show_help: false,
            show_stats: false,
            quit_requested: false,
            exit: false,
            is_mouse_grabbed: false,
            mouse_grabbed_flag: true,
            old_grab_status: false,
            tick_time,
            daylight_cycle: true
        }
    }

    pub(crate) fn is_mouse_grabbed(&self) -> bool {
        self.is_mouse_grabbed
    }

    fn grab_mouse(&self, grab: bool) {
        if grab {
            set_cursor_grab(true);
            show_mouse(false);
        } else {
            set_cursor_grab(false);
            show_mouse(true);
        }
    }

    pub(crate) fn toggle_mouse_grab(&mut self) {
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

    fn map_range(x: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> f32 {
        (x - x_min) * ((y_max - y_min) / (x_max - x_min)) + y_min
    }

    fn show_game_info(&mut self, props: UIProps) {
        let position = vec2(self.viewport_width - 400.0, 0.0);
        let size = vec2(400.0, 700.0);
        
        widgets::Window::new(
            hash!("game_info_window"),
            position,
            size
        )
        .label("Robot")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, format!("Game tick interval: ").as_str());
            ui.slider(hash!("tick_time_slider"), "[0.0 - 5.0]", 0.0..5.0, &mut self.tick_time.borrow_mut());
            ui.checkbox(hash!("daylight_cicle_checkbox"), "Daylight cycle", &mut self.daylight_cycle);
            ui.label(None, "Energy: ");
            let max_energy_level = 1000.0; //pub(crate) const MAX_ENERGY_LEVEL: usize = 1000;
            let cursor = ui.canvas().cursor();
            ui.canvas().rect(
                Rect::new(
                    cursor.x,
                    cursor.y,
                    Self::map_range(props.robot_energy as f32, 0.0, max_energy_level, 1.0, 300.0),
                    20.0
                ),
                BLACK,
                YELLOW,
            );
            ui.separator();

            ui.label(None, &format!("Coordinates X: {}, Y: {}", props.robot_coordinates.0, props.robot_coordinates.1));
            ui.label(None, &format!("Backpack size: {}", props.robot_backpack_size));

            let backpack_size = vec2(390.0, 400.0);
            let backpack_item_size = vec2(380.0, 22.0);
            widgets::Group::new(
                hash!("backpack_contents"),
                backpack_size
            )
            .layout(Layout::Vertical)
            .ui(ui, |ui| {
                for (index, (item, amount)) in props.robot_backpack_contents.iter().enumerate() {
                    widgets::Group::new(
                        hash!("backpack_item", index, amount),
                        backpack_item_size
                    )
                    .layout(Layout::Horizontal)
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
                        ui.same_line(150.0);
                        ui.label(None, &format!("{}", amount));
                    });
                }
            });

            ui.label(None, format!("Discoverable tiles: {}", props.discoverable_tiles).as_str());
            ui.label(None, format!("Score: {}", props.robot_score).as_str());
            ui.label(None, format!("Time of day: {}", match props.time_of_day {
                    DayTime::Morning => "Morning",
                    DayTime::Afternoon => "Afternoon",
                    DayTime::Night => "Night",
                }).as_str()
            );
            ui.label(None, format!("Time clock: {}", props.time_of_day_string).as_str());
            ui.label(None, format!("Weather: {}", match props.weather_condition {
                WeatherType::Sunny => "Sunny",
                WeatherType::Rainy => "Rainy",
                WeatherType::Foggy => "Foggy",
                WeatherType::TropicalMonsoon => "Tropical moonsoon",
                WeatherType::TrentinoSnow => "Trentino's snow",
                }).as_str()
            );
        });
    }

    fn show_stats(&self) {
        let position = vec2(0.0, 0.0);
        let size = vec2(200.0, 100.0);

        widgets::Window::new(
            hash!("stats_window"), 
            position, 
            size
        )
        .label("Statistics")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("FPS: {}", get_fps()));
            ui.label(None, &format!("Texture count: {}", textures_count()));
        });
    }

    fn show_help(&self) {
        let position = vec2(0.0, self.viewport_height - 200.0);
        let size = vec2(300.0, 200.0);

        widgets::Window::new(
            hash!("help_window"), 
            position,
            size
        )
        .label("Help")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, &format!("Toggle mouse grab: J"));
            ui.label(None, &format!("Toggle statistics window: F3"));
        });
    }

    fn show_exit_dialog(&mut self) {
        let position = vec2(self.viewport_width / 2.0 - 100.0, self.viewport_height / 2.0 - 50.0);
        let size = vec2(200.0, 100.0);

        widgets::Window::new(
            hash!("exit_dialog"),
            position,
            size
        )
        .label("Exit")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Do you really want to quit?");
            ui.separator();
            ui.same_line(60.0);
            if ui.button(None, "Yes") {
                self.exit = true;
            }
            ui.same_line(120.);
            if ui.button(None, "No") {
                if self.old_grab_status {
                    self.toggle_mouse_grab();
                }
                self.mouse_grabbed_flag = true;
                self.quit_requested = false;
            }
        });
    }

    pub fn exit(&self) -> bool {
        self.exit
    }

    pub fn is_day_light_cycle_on(&self) -> bool {
        self.daylight_cycle
    }

    pub(crate) fn render(&mut self, props: UIProps) {
        draw_text("Press H for help", 0.0, self.viewport_height, 30.0, GREEN);

        self.show_game_info(props);
        
        if self.quit_requested {
            if self.mouse_grabbed_flag {
                self.mouse_grabbed_flag = false;
                self.old_grab_status = self.is_mouse_grabbed;
            }
            if self.is_mouse_grabbed {
                self.toggle_mouse_grab();
            }
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