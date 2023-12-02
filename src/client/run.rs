use super::{input::Input, ClientState, render::Renderer, rsc::FRAME_TIME, update::update};
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub async fn run_client() {
    let event_loop = EventLoop::new();
    let mut renderer = Renderer::new(&event_loop, false).await;
    let frame_time = FRAME_TIME;
    let mut exit = false;
    let mut state = ClientState::new();

    let mut target = Instant::now();
    let mut input = Input::new();
    let mut prev_update = Instant::now();

    renderer.window.set_visible(true);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } if window_id == renderer.window.id() => {
                match event {
                    WindowEvent::CloseRequested => exit = true,
                    WindowEvent::Resized(_) => renderer.resize(),
                    _ => input.update(event),
                }
            }
            Event::RedrawRequested(_) => {
                renderer.draw();
            }
            Event::MainEventsCleared => {
                let now = Instant::now();
                if now > target {
                    target += frame_time;
                    let delta = now - prev_update;
                    prev_update = now;

                    exit |= update(&mut state, &input, &renderer, &delta);
                    input.end();
                    renderer.update(&state);
                    renderer.draw();

                    if exit {
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
            _ => {}
        }
    });
}
