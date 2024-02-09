use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;
use robotics_lib::{runner::{Robot, Runner}, world::world_generator::Generator};
use rip_worldgenerator::MyWorldGen;
use olympus::visualizer::{custom_camera::CustomCamera, gui::GUI, oracle::Oracle, renderer::Renderer};

use dummy_robot::DummyRobot;

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
    let mut world_generator = MyWorldGen::new_param(world_size, 5, 5, 5, true, false, 5, false, Some(25));
    // let world_copy = world_generator.gen();

    //Oracle
    let oracle = Rc::new(RefCell::new(Oracle::new()));

    //Robot
    let robot = Box::new(DummyRobot::new(Robot::default(), Rc::clone(&oracle)));

    //Game
    let mut runner = Runner::new(robot, &mut world_generator).expect("errore");
    let mut last_time = 0.0;
    #[allow(unused_assignments)]
    let mut current_time = 0.0;

    //Camera
    let mut camera = CustomCamera::default();

    //Renderer
    let renderer = Renderer::new(world_size);

    //GUI
    let mut gui = GUI::default();
    gui.init();

    loop {
        //Background
        Renderer::draw_background();

        //Input
        camera.handle_input();
        
        //Camera
        camera.update();
        set_camera(camera.get_actual_camera());

        //Game Tick
        current_time = get_time();
        if (current_time - last_time) > 0.0 {
            runner.game_tick().expect("Error during game tick");
            last_time = current_time;
        }
        
        //World Render
        renderer.render(oracle.borrow().get_render_props());

        //GUI
        set_default_camera();
        gui.draw(oracle.borrow().get_gui_props());

        next_frame().await
    }
}