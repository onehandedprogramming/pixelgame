#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
pub struct TileInstance {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
