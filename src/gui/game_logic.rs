use std::collections::HashMap;
use robotics_lib::{energy::{Energy}, runner::{backpack::BackPack, Runner}, world::coordinates::{Coordinate}};
use robotics_lib::world::tile::Content;

pub struct GameLogic {
    runner: Runner,
}

struct RobotStats<'a> {
    pub energy: &'a Energy,
    pub coordinate: &'a Coordinate,
    pub backpack: &'a BackPack,
}

impl<'a> RobotStats<'a> {
    fn new(energy: &'a Energy, coordinate: &'a Coordinate , backpack: &'a BackPack) -> Self {       
        Self {
            energy,
            coordinate,
            backpack,
        }
    }
}

pub struct ConvertedStats {
    pub energy: usize, 
    pub coordinates: (usize, usize),
    pub backpack_contents: HashMap<Content, usize>,
    pub backpack_size: usize
}

impl ConvertedStats {
    fn new(energy: usize, coordinates: (usize, usize), backpack_contents: HashMap<Content, usize>, backpack_size: usize ) -> Self {
        Self {
            energy,
            coordinates,
            backpack_contents,
            backpack_size
        }
    }

    fn convert_stats(stats: RobotStats) -> ConvertedStats {
        ConvertedStats::new(
            stats.energy.get_energy_level(),
            (stats.coordinate.get_row(), stats.coordinate.get_col()),
            stats.backpack.get_contents().clone(),
            stats.backpack.get_size()
        )
    }
}

impl Default for ConvertedStats {
    fn default() -> Self {
        Self {
            energy: 0,
            coordinates: (0, 0),
            backpack_contents: HashMap::default(),
            backpack_size: 0
        }
    }
}

impl GameLogic {
    pub fn new(runner: Runner) -> Self {
        Self {
            runner
        }
    }

    pub fn get_robot_stats(&self) -> ConvertedStats {
        ConvertedStats::convert_stats(        
            RobotStats::new(
                self.runner.get_robot().get_energy(),
                self.runner.get_robot().get_coordinate(),
                self.runner.get_robot().get_backpack(),
            ))
    }

    pub fn tick(&mut self) {
        // Impossible because i cannot change the definition of runner
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