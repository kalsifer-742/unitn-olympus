use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};
use macroquad::hash;

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

    pub fn draw(&mut self, props: &Props) {
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., ORANGE);

        widgets::Window::new(
            hash!(),
            vec2(self.window_width as f32 - 400., 0.),
            vec2(400., 400.)
        )
        .label("Robot")
        .titlebar(true)
        .ui(&mut root_ui(), |ui| {
            let mut canvas = ui.canvas();
            let cursor = canvas.cursor();

            canvas.rect(
                Rect::new(cursor.x, cursor.y, 200.0, 18.0),
                Color::new(0.0, 0.0, 0.0, 1.0),
                BLUE,
            );
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