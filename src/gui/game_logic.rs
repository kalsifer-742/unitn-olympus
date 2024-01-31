use robotics_lib::runner::Runner;

pub struct GameLogic {
    runner: Runner,
}

impl GameLogic {
    pub fn new(runner: Runner) -> Self {
        Self {
            runner
        }
    }

    pub fn tick(&mut self) {
        // Impossibile because i cannot change the definition of runner
        // so that the robot implements a specific trait
        // self.runner.get_robot().offering_to_the_gods();
        
        // Also impossible because the original trait does not implement Any
        // if let Some(believer) = self.runner.get_robot().as_ref().downcast_ref::<DummyRobot>() {
        //     believer.offering_to_the_gods();
        // } else {
        //     println!("The robot is not a Believer!");
        // }
        
        self.runner.game_tick().expect("Error during game tick");
    }
}