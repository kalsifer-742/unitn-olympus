use std::rc::Rc;
use std::cell::RefCell;
use dummy_robot::DummyRobot;
use macroquad::prelude::*;
use olympus::gui::{GUI};
use olympus::gui::game_logic::GameLogic;
use rip_worldgenerator::MyWorldGen;
use robotics_lib::runner::{Robot, Runner};

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
    let gui = Rc::new(RefCell::new(GUI::default()));
    let world_size = 25;
    let mut world_generator = MyWorldGen::new_param(world_size, 5, 5, 5, true, false, 5);
    let robot = DummyRobot::new(Robot::default(), gui.clone());
    let mut game_logic = GameLogic::new(
        Runner::new(Box::new(robot), &mut world_generator).expect("Error creating runner")
    );

    gui.borrow_mut().init(world_size);
    let mut last_time = 0.0;

    loop {
        //Background
        GUI::draw_background();
        //Input
        gui.borrow_mut().handle_input();
        //Camera
        set_camera(gui.borrow_mut().camera.get_actual_camera());

        let current_time = get_time();
        
        if current_time - last_time > 0.2 {
            game_logic.tick();
            last_time = current_time;
        }
        
        gui.borrow().draw_world();

        set_default_camera();
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., ORANGE);

        next_frame().await
    }
}