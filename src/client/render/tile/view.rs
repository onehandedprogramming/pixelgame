use crate::{util::point::Point, client::render::{uniform::UniformData, RenderUpdateData}};

#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable)]
pub struct TileView {
    pub pos: Point<f32>,
    pub proj: Point<f32>,
    pub width: u32,
}

unsafe impl bytemuck::Pod for TileView {}

impl UniformData for TileView {
    fn update(&mut self, data: &RenderUpdateData) -> bool {
        let new = TileView {
            pos: -data.state.camera.pos,
            proj: data.state.camera.proj_for(data.size),
            width: data.state.width
        };
        if *self == new {
            false
        } else {
            *self = new;
            true
        }
    }
}

impl Default for TileView {
    fn default() -> Self {
        Self { pos: Point::zero(), proj: Point::zero(), width: 0 }
    }
}

