use std::rc::Rc;
use std::cell::RefCell;
use dummy_robot::DummyRobot;
use macroquad::prelude::*;
use olympus::gui::{GUI};
use olympus::gui::game_logic::GameLogic;
use rip_worldgenerator::MyWorldGen;
use robotics_lib::runner::{Robot, Runner};
use olympus::gui::ui::Props;

mod dummy_robot;

fn window_conf() -> Conf {
    Conf {
        window_title: "Olympus".to_owned(),
        window_width: 1902,
        window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //World Generator
    let world_size = 50;
    let mut world_generator = MyWorldGen::new_param(world_size, 5, 5, 5, true, false, 5, true, Some(25));

    //Robot
    //this is not acceptable to me and needs to be fixed somehow
    let gui = Rc::new(RefCell::new(GUI::default()));
    let robot = DummyRobot::new(Robot::default(), gui.clone());

    //Game
    let mut game_logic = GameLogic::new(
        Runner::new(Box::new(robot), &mut world_generator).expect("Error creating runner")
    );
    let mut last_time = 0.0;
    let mut current_time = 0.0;

    //GUI
    //let gui = Rc::new(RefCell::new(GUI::default()));
    gui.borrow_mut().init(world_size);
    let mut props = Props::new(game_logic.get_robot_stats());

    loop {
        //Background
        GUI::draw_background();

        //Input
        gui.borrow_mut().handle_input();
        
        //Camera
        set_camera(gui.borrow_mut().camera.get_actual_camera());

        //Game Tick
        current_time = get_time();
        if current_time - last_time > 1.0 {
            game_logic.tick();
            props.update(game_logic.get_robot_stats());

            last_time = current_time;
        }
        
        //World
        gui.borrow().draw_world();

        //UI
        set_default_camera();
        gui.borrow_mut().draw_ui(&props);

        next_frame().await
    }
}