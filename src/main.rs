use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;
use oracle::Oracle;
use robotics_lib::runner::Robot;
use rip_worldgenerator::MyWorldGen;
use olympus::visualizer::{Visualizer};

use runner_wrapper::RunnerWrapper;
use dummy_robot::DummyRobot;

mod oracle;
mod runner_wrapper;
mod dummy_robot;

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
    // World Generator
    let world_size = 200;
    let mut world_generator = MyWorldGen::new_param(world_size, 5, 5, 5, true, false, 5, false, Some(25));
    // Oracle
    let oracle = Rc::new(RefCell::new(Oracle::default()));
    // Robot
    let robot = Box::new(DummyRobot::new(Robot::default(), Rc::clone(&oracle)));
    // Game
    let mut game: RunnerWrapper = RunnerWrapper::new(robot, &mut world_generator);
    // Visualizer
    let mut visualizer = Visualizer::new(world_size);
    visualizer.init();

    loop {
        //Input
        visualizer.handle_input();
        if visualizer.gui.exit() {
            break;
        }
        
        //Game
        game.tick();
        
        //Visualizer
        visualizer.show(oracle.borrow().get_props(), game.get_tick_time());

        next_frame().await
    }
}