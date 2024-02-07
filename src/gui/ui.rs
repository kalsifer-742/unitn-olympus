use std::hash;
use std::io::Cursor;

use macroquad::{prelude::*, ui};
use macroquad::ui::widgets::Group;
use macroquad::ui::{root_ui, widgets};
use macroquad::hash;
use robotics_lib::world::tile::Content;

use super::game_logic::ConvertedStats;

pub(crate) struct UI {
    window_width: f32,
    window_height: f32, 
}

pub struct Props {
    stats: ConvertedStats
}

impl Props {
    pub fn new(stats: ConvertedStats) -> Self {
        Self {
            stats
        }
    }

    pub fn update(&mut self, stats: ConvertedStats) {
        self.stats = stats;
    }
}

impl UI {
    fn new(window_width: f32, window_height: f32) -> Self {
        Self {
            window_width,
            window_height,
        }
    }

    fn map_range(x: f32, x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> f32 {
        (x - x_min) * ((y_max - y_min) / (x_max - x_min)) + y_min
    }


    pub fn draw(&mut self, props: &Props) {
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
                    Self::map_range(props.stats.energy as f32, 0., 1000., 0., 100.),
                    20.0),
                Color::new(0.0, 0.0, 0.0, 1.0),
                YELLOW,
            );

            ui.label(None, &format!("Coordinates X: {}, Y: {}", props.stats.coordinates.0, props.stats.coordinates.1));
            
            ui.separator();

            widgets::Group::new(hash!("backpack"), vec2(350., 200.))
            .ui(ui, |ui| {
                for (index, (item, amount)) in props.stats.backpack_contents.iter().enumerate() {
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

            ui.label(None, &format!("Backpack size: {}", props.stats.backpack_size));
        });
    }
}

impl Default for UI {
    fn default() -> Self {
        Self {
            window_width: 1920.0,
            window_height: 1080.0,
        }
    }
}