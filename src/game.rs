use midgard::world_generator::{WorldGenerator, WorldGeneratorParameters};
use robotics_lib::runner::{Robot, Runner};
use crate::dummy_robot::DummyRobot;

struct Game {
    world_gen_params: WorldGeneratorParameters,
    robot: Robot,
    runner: Runner,
}

impl Game {
    fn init(&self) {
        let mut world_generator = WorldGenerator::new(self.world_gen_params);
        let robot = DummyRobot::default();
        let mut game = Runner::new(Box::new(robot), &mut world_generator).expect("Error during runner creation");
    }

    fn tick() {

    }
}