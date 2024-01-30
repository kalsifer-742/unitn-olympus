use macroquad::prelude::*;
use midgard::world_generator::{WorldGenerator, WorldGeneratorParameters};
use robotics_lib::runner::Runner;
use robotics_lib::world::tile::Tile;
use crate::dummy_robot::DummyRobot;
use crate::camera::CameraConfig;
use crate::input::handle_keys;

mod dummy_robot;
mod camera;
mod input;
mod game;

struct WorldMap {
    map_size: u64,
    explored_map: Vec<Vec<Tile>>,
}

struct UI {
    map: WorldMap
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Olympus".to_owned(),
        fullscreen: true,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let world_seed = 15;
    let world_size = 10;

    let mut camera_config = CameraConfig::new(
        vec3(-20., 10., 0.),
        vec3(0., 0., 0.),
        vec3(0., 30., 0.),
    );



    loop {
        //Background
        clear_background(LIGHTGRAY);
        //Input
        handle_keys(&mut camera_config);
        //Camera
        set_camera(&Camera3D {
            position: camera_config.position,
            target: camera_config.target,
            up: camera_config.up,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);
        draw_affine_parallelepiped(Vec3::ZERO, 3. * Vec3::X, 2. * Vec3::Y, 5. * Vec3::Z, None, RED);

        game.game_tick().expect("Error during game tick");

        next_frame().await
    }
}