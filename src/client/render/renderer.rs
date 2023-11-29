use super::tile::TilePipeline;
use crate::client::{rsc::CLEAR_COLOR, ClientState};
use winit::{
    event_loop::EventLoop,
    window::{Fullscreen, Window, WindowBuilder},
};

pub struct Renderer {
    pub window: Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    encoder: Option<wgpu::CommandEncoder>,
    staging_belt: wgpu::util::StagingBelt,
    tile_pipeline: TilePipeline,
}

impl Renderer {
    pub async fn new(event_loop: &EventLoop<()>, fullscreen: bool) -> Self {
        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        if fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(None)))
        }

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = unsafe {
            instance
                .create_surface(&window)
                .expect("Could not create window surface!")
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Could not get adapter!");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .expect("Could not get device!");

        // TODO: use a logger
        let info = adapter.get_info();
        println!("Adapter: {}", info.name);
        println!("Backend: {:?}", info.backend);

        let surface_caps = surface.get_capabilities(&adapter);
        // Set surface format to srbg
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        // create surface config
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);
        // not exactly sure what this number should be,
        // doesn't affect performance much and depends on "normal" zoom
        let staging_belt = wgpu::util::StagingBelt::new(4096 * 4);

        Self {
            tile_pipeline: TilePipeline::new(&device, &config.format),
            window,
            encoder: None,
            staging_belt,
            surface,
            device,
            adapter,
            config,
            queue,
        }
    }

    pub fn start_encoder(&mut self) {
        self.encoder = Some(
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                }),
        );
    }

    pub fn draw(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.encoder.take().expect("encoder not started");
        {
            let render_pass = &mut encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(CLEAR_COLOR),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            self.tile_pipeline.draw(render_pass);
        }

        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        self.staging_belt.recall();
    }

    pub fn update<'a>(&mut self, state: &ClientState) {
        let size = &self.window.inner_size();
        if size.width != self.config.width || size.height != self.config.height {
            self.resize();
        }

        let mut encoder = self.encoder.take().expect("encoder not started");
        let camera_view = self.tile_pipeline.update(
            &self.device,
            &mut encoder,
            &mut self.staging_belt,
            &state.camera,
            size,
        );
        self.encoder = Some(encoder);

        camera_view
    }

    pub fn resize(&mut self) {
        let size = self.window.inner_size();
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }
}
