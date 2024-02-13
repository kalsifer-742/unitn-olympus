use std::{cell::RefCell, rc::Rc};

use channel::Channel;
use macroquad::prelude::*;
use robotics_lib::{runner::Runnable, world::world_generator::Generator};
use gui::GUI;
use runner_wrapper::RunnerWrapper;

mod gui;
mod runner_wrapper;
pub mod channel;

pub struct Visualizer {
    runner: RunnerWrapper,
    gui: GUI,
    channel: Rc<RefCell<Channel>>
}

impl Visualizer {
    pub fn new(robot: impl Runnable + 'static, world_generator: impl Generator, channel: Rc<RefCell<Channel>>) -> Self {
        Self {
            runner: RunnerWrapper::new(robot, world_generator),
            gui: Default::default(),
            channel
        }
    }

    pub async fn start(&mut self) {
        self.gui.ui.grab_mouse(true);

        loop {
            self.gui.handle_input();
            if self.gui.ui.exit() {
                break;
            }

            self.runner.tick();
            //get robot data
            
            let props = self.channel.borrow().receive();
            self.gui.render(props, self.runner.get_tick_time());
    
            next_frame().await
        }
    }
}