use super::{camera::Camera, render::tile::TileInstance};

pub struct ClientState {
    pub camera: Camera,
    pub camera_scroll: f32,
    pub grid: [TileInstance; 100 * 100],
    pub width: u32,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            camera_scroll: 0.0,
            grid: [TileInstance {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }; 100 * 100],
            width: 100,
        }
    }
}
