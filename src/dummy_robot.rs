use std::cell::RefCell;
use std::rc::Rc;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{Direction, go, robot_map, robot_view};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use olympus::gui::GUI;

pub struct DummyRobot{
    robot: Robot,
    gui: Rc<RefCell<GUI>>,
}

impl DummyRobot {
    pub fn new(robot: Robot, gui: Rc<RefCell<GUI>>) -> Self {
        Self {
            robot,
            gui
        }
    }
}

// impl Default for DummyRobot {
//     fn default() -> Self {
//         Self {
//             robot: Default::default(),
//             gui: GUI::default()
//         }
//     }
// }

impl Runnable for DummyRobot {
    fn process_tick(&mut self, world: &mut World) {
        println!("VIEW AROUND");
        let _robot_view= robot_view(self, world);
        println!("GET MAP");
        let robot_world = robot_map(world).expect("Problem calling robot_map (probably Mutex problems)");
        println!("MOVE AROUND");
        
        self.gui.borrow_mut().offering_to_the_gods(robot_world);

        //sleep(time::Duration::from_millis(500));

        if go(self, world, Direction::Left).is_err() {
            return;
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Ready => {}
            Event::Terminated => {}
            Event::TimeChanged(_) => {}
            Event::DayChanged(_) => {}
            Event::EnergyRecharged(_) => {}
            Event::EnergyConsumed(_) => {}
            Event::Moved(_, _) => {}
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

// impl Believer for DummyRobot {
//     fn offering_to_the_gods() {
        
//     }
// }