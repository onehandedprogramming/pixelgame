use std::marker::PhantomData;
use wgpu::VertexAttribute;

pub struct Instances<T: bytemuck::Pod> {
    len: usize,
    buffer: wgpu::Buffer,
    location: u32,
    attrs: [VertexAttribute; 1],
    label: String,
    data: PhantomData<T>,
}

impl<T: bytemuck::Pod> Instances<T> {
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        data: &[T],
    ) {
        if data.len() != self.len {
            self.len = data.len();
            self.buffer = Self::init_buf(device, &self.label, self.len);
        }
        if self.len == 0 {
            return;
        }
        // TODO: "damage tracking" ?
        let mut view = belt.write_buffer(
            encoder,
            &self.buffer,
            0,
            unsafe {
                std::num::NonZeroU64::new_unchecked((self.len * std::mem::size_of::<T>()) as u64)
            },
            device,
        );
        view.copy_from_slice(bytemuck::cast_slice(data));
    }

    pub fn init(
        device: &wgpu::Device,
        label: &str,
        location: u32,
        format: wgpu::VertexFormat,
    ) -> Self {
        Self {
            len: 0,
            buffer: Self::init_buf(device, label, 0),
            location,
            attrs: [wgpu::VertexAttribute {
                format,
                offset: 0,
                shader_location: location,
            }],
            label: label.to_string(),
            data: PhantomData {},
        }
    }

    fn init_buf(device: &wgpu::Device, label: &str, size: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&(label.to_owned() + " Instance Buffer")),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            size: (size * std::mem::size_of::<T>()) as u64,
            mapped_at_creation: false,
        })
    }

    pub fn set_in<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(self.location, self.buffer.slice(..));
    }

    pub fn desc(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<T>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &self.attrs,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
