use wgpu::util::DeviceExt;

pub struct Uniform<T: bytemuck::Pod + PartialEq> {
    data: T,
    buffer: wgpu::Buffer,
    binding: u32,
}

impl<T: Default + PartialEq + bytemuck::Pod> Uniform<T> {
    pub fn init(device: &wgpu::Device, name: &str, binding: u32) -> Self {
        let data = T::default();
        Self {
            data,
            buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(name),
                contents: bytemuck::cast_slice(&[data]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }),
            binding,
        }
    }
}

impl<T: PartialEq + bytemuck::Pod> Uniform<T> {
    pub fn bind_group_entry(&self) -> wgpu::BindGroupEntry {
        return wgpu::BindGroupEntry {
            binding: self.binding,
            resource: self.buffer.as_entire_binding(),
        };
    }
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        data: T,
    ) {
        if data != self.data {
            self.data = data;
            let slice = &[data];
            let mut view = belt.write_buffer(
                encoder,
                &self.buffer,
                0,
                unsafe {
                    std::num::NonZeroU64::new_unchecked(
                        (slice.len() * std::mem::size_of::<T>()) as u64,
                    )
                },
                device,
            );
            view.copy_from_slice(bytemuck::cast_slice(slice));
        }
    }
}
