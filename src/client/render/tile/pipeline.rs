use super::{instance::TileInstance, view::TileView};
use crate::client::render::{instance::Instances, uniform::Uniform, RenderUpdateData};

pub struct TilePipeline {
    pipeline: wgpu::RenderPipeline,
    view: Uniform<TileView>,
    instances: Instances<TileInstance>,
    bind_group: wgpu::BindGroup,
}

impl TilePipeline {
    pub fn new(device: &wgpu::Device, format: &wgpu::TextureFormat) -> Self {
        // shaders
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Tile Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let view = Uniform::<TileView>::init(device, "Tile View Uniform", 0);
        let instances = Instances::<TileInstance>::init(
            device,
            "Tile Instance",
            0,
            wgpu::VertexFormat::Float32x4,
        );

        // bind groups
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("tile_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[view.bind_group_entry()],
            label: Some("tile_bind_group"),
        });

        // pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Tile Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Tile Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[instances.desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: *format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            pipeline: render_pipeline,
            view,
            instances,
            bind_group,
        }
    }

    pub fn update<'a>(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        belt: &mut wgpu::util::StagingBelt,
        update_data: &RenderUpdateData,
    ) {
        self.instances
            .update(device, encoder, belt, &update_data.state.grid);
        self.view.update(device, encoder, belt, update_data);
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        self.instances.set_in(render_pass);
        render_pass.draw(0..4, 0..self.instances.len() as u32);
    }
}
