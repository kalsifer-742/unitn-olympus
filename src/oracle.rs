use robotics_lib::interface::{get_score, robot_map};
use robotics_lib::runner::Runnable;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::World;

use olympus::visualizer::VisualizerProps;

// Impossible because I cannot change the definition of runner
// so that the robot implements a specific trait
// self.runner.get_robot().offering_to_the_gods();

// Also impossible because the original trait does not implement Any
// if let Some(believer) = self.runner.get_robot().as_ref().downcast_ref::<DummyRobot>() {
//     believer.offering_to_the_gods();
// } else {
//     println!("The robot is not a Believer!");
// }

// let _world = self.demigod.offering_to_the_gods();

// pub trait Demigod: Runnable {
//     fn offering_to_the_gods(&self, oracle: &mut Oracle, r_props: RendererProps, g_props: GUIProps);
// }

pub struct Oracle {
    props: VisualizerProps,
    tick_time: f64
}

impl Oracle {
    pub fn get_props(&self) -> &VisualizerProps {
        &self.props
    }

    pub fn update_props(&mut self, robot: & impl Runnable, world: &mut World) {
        self.props.explored_world_map = robot_map(world).expect("Problem calling robot_map (probably Mutex problems)");
        self.props.robot_coordinates = (robot.get_coordinate().get_row(), robot.get_coordinate().get_col());
        self.props.robot_energy = robot.get_energy().get_energy_level();
        self.props.robot_backpack_contents = robot.get_backpack().get_contents().clone();
        self.props.robot_backpack_size = robot.get_backpack().get_size();
        self.props.discoverable_tiles = world.get_discoverable();
        self.props.robot_score = get_score(world);
    }

    pub fn update_weather(&mut self, weather: EnvironmentalConditions) {
        self.props.time_of_day = weather.get_time_of_day();
        self.props.time_of_day_string = weather.get_time_of_day_string();
        self.props.weather_condition = weather.get_weather_condition();
    }

    pub fn get_tick_time(&self) -> f64 {
        self.tick_time
    }

    pub fn change_tick_time(&mut self, tick_time: f64) -> Result<f64, String>{
        if tick_time > 0.0 {
            self.tick_time = tick_time;
            Ok(self.tick_time)
        } else {
            Err("tick_time bust be greater than 0.0".to_string())
        }
    }
}

impl Default for Oracle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            tick_time: 0.5,
        }
    }
}