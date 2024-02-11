use macroquad::time::get_time;
use robotics_lib::{runner::{Runnable, Runner}, world::world_generator::Generator};

pub struct RunnerWrapper {
    runner: Runner,
    last_time: f64,
    current_time: f64,
    tick_time: f32
}

impl RunnerWrapper {
    pub fn new(robot: Box<dyn Runnable>, world_generator: &mut impl Generator) -> Self {
        Self {
            runner: Runner::new(robot, world_generator).expect("errore"),
            last_time: get_time(),
            current_time: get_time(),
            tick_time: 0.5
        }
    }

    pub fn get_tick_time(&mut self) -> &mut f32 {
        &mut self.tick_time
    }

    #[allow(dead_code)]
    pub fn change_tick_time(&mut self, tick_time: f32) -> Result<f32, String>{
        if tick_time > 0.0 {
            self.tick_time = tick_time;
            Ok(self.tick_time)
        } else {
            Err("tick_time bust be greater than 0.0".to_string())
        }
    }

    pub fn tick(&mut self) {
        self.current_time = get_time();
        
        if (self.current_time - self.last_time) > self.tick_time as f64 {
            self.runner.game_tick().expect("Error during game tick");
            self.last_time = self.current_time;
        }
    }
}