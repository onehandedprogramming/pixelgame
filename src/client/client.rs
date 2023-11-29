use super::{camera::Camera, rsc::FRAME_TIME};
use super::{input::Input, render::Renderer};
use std::time::{Duration, Instant};
use winit::{
    event::{Event, VirtualKeyCode as Key, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

struct Client {
    renderer: Renderer,
    pub frame_time: Duration,
    pub exit: bool,
    pub state: ClientState,
}

impl Client {
    pub async fn new(event_loop: &EventLoop<()>) -> Self {
        let fullscreen = false;
        Self {
            renderer: Renderer::new(event_loop, fullscreen).await,
            frame_time: FRAME_TIME,
            exit: false,
            state: ClientState::new(),
        }
    }

    fn resize(&mut self) {
        self.renderer.resize();
        self.renderer.start_encoder();
        self.renderer.draw();
    }

    fn render(&mut self) {
        self.renderer.start_encoder();
        self.renderer.update(&self.state);
        self.renderer.draw();
    }

    fn handle_input(&mut self, delta: &Duration, input: &Input) {
        if input.just_pressed(Key::Escape) {
            self.exit = true;
        }
        if input.scroll_delta != 0.0 {
            self.state.camera_scroll += input.scroll_delta;
            self.state.camera.scale = (self.state.camera_scroll * 0.1).exp();
        }
        let delta_mult = delta.as_millis() as f32;
        let move_dist = 0.02 * delta_mult / self.state.camera.scale;
        let pos = &mut self.state.camera.pos;
        if input.pressed(Key::W) {
            pos.y += move_dist;
        }
        if input.pressed(Key::A) {
            pos.x -= move_dist;
        }
        if input.pressed(Key::R) {
            pos.y -= move_dist;
        }
        if input.pressed(Key::S) {
            pos.x += move_dist;
        }
    }
}

pub async fn run_client() {
    let event_loop = EventLoop::new();
    let mut client = Client::new(&event_loop).await;

    let mut target = Instant::now();
    let mut last_update = Instant::now();
    let mut input = Input::new();

    client.renderer.window.set_visible(true);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } if window_id == client.renderer.window.id() => {
                match event {
                    WindowEvent::CloseRequested => client.exit = true,
                    WindowEvent::Resized(_) => client.resize(),
                    _ => input.update(event),
                }
            }
            Event::RedrawRequested(_) => {
                client.renderer.start_encoder();
                client.renderer.draw();
            }
            Event::MainEventsCleared => {
                let now = Instant::now();
                if now > target {
                    target += client.frame_time;

                    let time_delta = now - last_update;
                    last_update = now;

                    client.handle_input(&time_delta, &input);
                    input.end();
                    // client.update(&input, now);
                    client.render();

                    if client.exit {
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
            _ => {}
        }
    });
}

pub struct ClientState {
    pub camera: Camera,
    pub camera_scroll: f32,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            camera_scroll: 0.0,
        }
    }
}
