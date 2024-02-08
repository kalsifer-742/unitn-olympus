use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use macroquad::hash;
use robotics_lib::world::tile::Content;

pub struct GUI {
    window_width: f32,
    window_height: f32,
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

    fn map_range(x: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> f32 {
        (x - x_min) * ((y_max - y_min) / (x_max - x_min)) + y_min
    }

    pub fn draw(&self, props: &GUIProps) {
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., ORANGE);

        widgets::Window::new(
            hash!(),
            vec2(self.window_width as f32 - 400., 0.),
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
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            window_width: 1920.0,
            window_height: 1080.0
        }
    }
}