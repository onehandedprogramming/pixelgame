use crate::{util::vector::Vec2, client::render::{uniform::UniformData, RenderUpdateData}};

#[repr(C)]
#[repr(align(16))]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable)]
pub struct TileView {
    pub pos: Vec2<f32>,
    pub proj: Vec2<f32>,
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
        Self { pos: Vec2::zero(), proj: Vec2::zero(), width: 0 }
    }
}

