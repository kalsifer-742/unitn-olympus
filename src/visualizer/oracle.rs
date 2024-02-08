use crate::visualizer::renderer::RendererProps;
use crate::visualizer::gui::GUIProps;

// Impossible because i cannot change the definition of runner
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
    renderer_props: RendererProps,
    gui_props: GUIProps
}

impl Oracle {
    pub fn new() -> Self {
        Self {
            renderer_props: Default::default(),
            gui_props: Default::default()
        }
    }

    pub fn update_renderer_props(&mut self, props: RendererProps) {
        self.renderer_props = props;
    }

    pub fn update_gui_props(&mut self, props: GUIProps) {
        self.gui_props = props;
    }

    pub fn get_render_props(&self) -> &RendererProps {
        &self.renderer_props
    }

    pub fn get_gui_props(&self) -> &GUIProps {
        &self.gui_props
    }

    pub fn update_props(&mut self, r_props: RendererProps, g_props: GUIProps) {
        self.update_renderer_props(r_props);
        self.update_gui_props(g_props);
    }
}