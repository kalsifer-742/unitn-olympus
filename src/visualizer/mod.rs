pub mod oracle;
mod controls;
mod renderer;
mod custom_camera;
mod gui;

use std::collections::HashMap;

use macroquad::prelude::*;
use renderer::{Renderer, RendererProps};
use gui::{GUI, GUIProps};
use robotics_lib::world::{environmental_conditions::{EnvironmentalConditions, WeatherType}, tile::{Content, Tile}};

use custom_camera::CustomCamera;

pub struct VisualizerProps {
    pub explored_world_map: Vec<Vec<Option<Tile>>>,
    pub robot_coordinates: (usize, usize),
    pub robot_energy: usize,
    pub robot_backpack_contents: HashMap<Content, usize>,
    pub robot_backpack_size: usize,
    pub discoverable_tiles: usize,
    pub robot_score: f32,
    pub weather: EnvironmentalConditions
}

impl Default for VisualizerProps {
    fn default() -> Self {
        Self {
            explored_world_map: vec![vec![None]],
            robot_coordinates: (0, 0),
            robot_energy: 0,
            robot_backpack_contents: HashMap::default(),
            robot_backpack_size: 0,
            discoverable_tiles: usize::MAX,
            robot_score: 0.0,
            weather: EnvironmentalConditions::new(
                &vec![WeatherType::Sunny],
                0,
                0
            ).unwrap(),
        }
    }
}

pub struct Visualizer {
    renderer: Renderer,
    camera: CustomCamera,
    pub gui: GUI,
}

impl Visualizer {
    pub fn new(world_map_size: usize) -> Self {
        Self {
            renderer: Renderer::new(world_map_size),
            camera: CustomCamera::default(),
            gui: Default::default(),
        }
    }

    pub fn init(&mut self) {
        self.gui.grab_mouse(true);
    }

    pub fn handle_input(&mut self) {
        self.camera.handle_input();
        self.gui.handle_input();
    }

    fn render_world(&self, props: &VisualizerProps) {
        self.renderer.render(
            &RendererProps { 
                explored_world_map: props.explored_world_map.clone(),
                robot_coordinates: props.robot_coordinates 
            }
        );
    }

    fn update_camera(&mut self) {
        self.camera.update();
        set_camera(self.camera.get_actual_camera());
    }

    fn show_gui(&mut self, props: &VisualizerProps) {
        set_default_camera();
        self.gui.show(
            &GUIProps { 
                robot_coordinates: props.robot_coordinates,
                robot_energy: props.robot_energy,
                robot_backpack_contents: props.robot_backpack_contents.clone(),
                robot_backpack_size: props.robot_backpack_size,
                discoverable_tiles: props.discoverable_tiles,
                robot_score: props.robot_score,
            }
        );
    }

    pub fn show(&mut self, props: &VisualizerProps) {
        self.update_camera(); // This needs to be done first
        self.render_world(props);
        self.show_gui(props);
    }
}