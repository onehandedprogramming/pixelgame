use crate::world::World;

use super::{camera::Camera, render::tile::TileInstance};

pub struct ClientState {
    pub camera: Camera,
    pub camera_scroll: f32,
    pub mouse_mode: MouseMode,
    pub grid: Vec<TileInstance>,
    pub width: u32,
    pub world: World,
}

impl ClientState {
    pub fn new() -> Self {
        let world = World::new();
        Self {
            camera: Camera::default(),
            camera_scroll: 0.0,
            mouse_mode: MouseMode::Dens,
            grid: vec![TileInstance {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }; world.size().area()],
            width: world.size().x as u32,
            world,
        }
    }
}

pub enum MouseMode {
    Dens,
    Vel,
}
