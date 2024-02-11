use macroquad::time::get_time;
use robotics_lib::{runner::{Runnable, Runner}, world::world_generator::Generator};

pub struct RunnerWrapper {
    runner: Runner,
    last_time: f64,
    current_time: f64,
}

impl RunnerWrapper {
    pub fn new(robot: Box<dyn Runnable>, world_generator: &mut impl Generator) -> Self {
        Self {
            runner: Runner::new(robot, world_generator).expect("errore"),
            last_time: get_time(),
            current_time: get_time(),
        }
    }

    pub fn tick(&mut self, tick_time: f64) {
        self.current_time = get_time();
        
        if (self.current_time - self.last_time) > tick_time {
            self.runner.game_tick().expect("Error during game tick");
            self.last_time = self.current_time;
        }
    }
}