use super::{camera::Camera, render::tile::TileInstance, world::World};

pub struct ClientState {
    pub camera: Camera,
    pub camera_scroll: f32,
    pub mouse_mode: MouseMode,
    pub grid: Vec<TileInstance>,
    pub world: World,
    pub width: u32,
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
            }; world.u.len()],
            width: 100,
            world,
        }
    }
}

pub enum MouseMode {
    Dens,
    Vel,
}
