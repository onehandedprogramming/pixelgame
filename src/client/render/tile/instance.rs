#[repr(C)]
#[derive(Clone, Copy, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
pub struct TileInstance {
    pub rgba: [f32; 4],
}
