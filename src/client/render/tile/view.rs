use winit::dpi::PhysicalSize;

use crate::{util::point::Point, client::camera::Camera};

const DEFAULT_SCALE: f32 = 0.02;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
pub struct TileView {
    pos: Point<f32>,
    proj: Point<f32>,
    width: u32,
    padding: u32,
}

impl Default for TileView {
    fn default() -> Self {
        Self { pos: Point::zero(), proj: Point::zero(), width: 0, padding: 0 }
    }
}

impl TileView {
    pub fn new(camera: &Camera, size: &PhysicalSize<u32>, width: u32) -> Self {
        Self {
            pos: -camera.pos,
            proj: Self::calc_proj(camera, size),
            width,
            padding: 0
        }
    }

    pub fn world_dimensions(&self) -> (f32, f32) {
        (2.0 / self.proj.x, 2.0 / self.proj.y)
    }

    pub fn render_to_world(&self, coords: Point<f32>) -> Point<f32> {
        coords / self.proj + self.pos
    }

    pub fn world_to_render(&self, coords: Point<f32>) -> Point<f32> {
        (coords - self.pos) * self.proj
    }

    fn calc_proj(camera: &Camera, size: &PhysicalSize<u32>) -> Point<f32> {
        let win_aspect = size.width as f32 / size.height as f32;
        let mut proj = if win_aspect > camera.aspect {
            Point::new(1.0, win_aspect)
        } else {
            Point::new(camera.aspect / win_aspect, camera.aspect)
        };
        proj *= camera.scale * DEFAULT_SCALE;
        proj
    }
}
