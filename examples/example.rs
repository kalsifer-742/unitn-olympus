use std::cell::RefCell;
use std::rc::Rc;

use macroquad::{prelude::*, rand::ChooseRandom};
use rip_worldgenerator::MyWorldGen;
use olympus::channel::Channel;
use olympus::Visualizer;
use robotics_lib::{energy::Energy, event::events::Event, interface::{go, Direction}, runner::{backpack::BackPack, Robot, Runnable}, world::{coordinates::Coordinate, World}};

pub struct DummyRobot{
    robot: Robot,
    channel: Rc<RefCell<Channel>>
}

impl DummyRobot {
    pub fn new(channel: Rc<RefCell<Channel>>) -> DummyRobot {
        DummyRobot {
            robot: Robot::default(),
            channel
        }
    }
}

impl Runnable for DummyRobot {
    fn process_tick(&mut self, world: &mut World) {
        // move and view around
        let directions = vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down];
        // go inside calls robot_view
        if go(self, world, directions.choose().unwrap().clone()).is_err() {
            return;
        }

        self.channel.borrow_mut().send_game_info(self, world);
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Ready => {}
            Event::Terminated => {}
            Event::TimeChanged(weather) => {
                self.channel.borrow_mut().send_weather_info(weather);
            }
            Event::DayChanged(_) => {}
            Event::EnergyRecharged(_) => {}
            Event::EnergyConsumed(_) => {}
            Event::Moved(_, (_, _)) => {}
            Event::TileContentUpdated(_, _) => {}
            Event::AddedToBackpack(_, _) => {}
            Event::RemovedFromBackpack(_, _) => {}
        }
    }

    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Olympus".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let channel = Rc::new(RefCell::new(Channel::default()));

    // World Generator
    let world_size = 200;
    let world_generator = MyWorldGen::new_param(world_size, 5, 5, 5, true, false, 5, false, Some(25));
    // Robot
    let robot = DummyRobot::new(Rc::clone(&channel));
    
    // Visualizer
    let mut visualizer = Visualizer::new(robot, world_generator, Rc::clone(&channel));
    visualizer.start().await
}