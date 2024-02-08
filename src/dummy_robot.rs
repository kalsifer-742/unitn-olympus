use std::cell::RefCell;
use std::rc::Rc;

use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{Direction, go, robot_map, robot_view};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;

use macroquad::rand::ChooseRandom;

use olympus::visualizer::oracle::Oracle;
use olympus::visualizer::renderer::RendererProps;
use olympus::visualizer::gui::GUIProps;

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
        //view around
        let _robot_view= robot_view(self, world);
        //get explored world
        let world_map = robot_map(world).expect("Problem calling robot_map (probably Mutex problems)");
        let world_map_size = world_map.len();

        //Renderer & GUI
        let r_props = RendererProps {
            world_map,
            world_map_size
        };
        let g_props = GUIProps {
            energy: self.get_energy().get_energy_level(),
            coordinates: (self.get_coordinate().get_row(), self.get_coordinate().get_col()),
            backpack_contents: self.get_backpack().get_contents().clone(),
            backpack_size: self.get_backpack().get_size(),
        };
        self.oracle.borrow_mut().update_props(r_props, g_props);

        //move around
        let directions = vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down];
        if go(self, world, directions.choose().unwrap().clone()).is_err() {
            return;
        }
    }

    fn handle_event(&mut self, event: Event) {
        //could be used but it makes things more complicated
        match event {
            Event::Ready => {}
            Event::Terminated => {}
            Event::TimeChanged(_) => {}
            Event::DayChanged(_) => {}
            Event::EnergyRecharged(_) => {}
            Event::EnergyConsumed(_) => {}
            Event::Moved(_tile, (_x, _y)) => {}
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