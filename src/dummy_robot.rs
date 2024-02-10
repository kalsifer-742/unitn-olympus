use std::cell::RefCell;
use std::rc::Rc;

use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{go, Direction};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;

use macroquad::rand::ChooseRandom;

use olympus::visualizer::oracle::Oracle;

pub struct DummyRobot{
    robot: Robot,
    oracle: Rc<RefCell<Oracle>>
}

impl DummyRobot {
    pub fn new(robot: Robot, oracle: Rc<RefCell<Oracle>> ) -> DummyRobot {
        DummyRobot {
            robot,
            oracle
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

        self.oracle.borrow_mut().update_props(self, world);
    }

    fn handle_event(&mut self, event: Event) {
        //could be used but it makes things more complicated
        match event {
            Event::Ready => {}
            Event::Terminated => {}
            Event::TimeChanged(weather) => {
                self.oracle.borrow_mut().update_weather(weather);
            }
            Event::DayChanged(_) => {}
            Event::EnergyRecharged(_) => {}
            Event::EnergyConsumed(_) => {}
            Event::Moved(_tile, (_x, _y)) => {
                // the event contains only the tile on which the robot is
                // not useful because there is no way to get the explored tiles
                // robot_view() gives a 3x3 matrix
                // inside this function i do not have acces to the world 
            }
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