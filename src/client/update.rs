use std::time::Duration;

use crate::util::point::Point;

use super::{
    input::Input,
    render::{tile::TileInstance, Renderer},
    ClientState, MouseMode,
};
use winit::event::VirtualKeyCode as Key;

pub fn update(
    state: &mut ClientState,
    input: &Input,
    renderer: &Renderer,
    t_delta: &Duration,
) -> bool {
    let cursor_pos = state
        .camera
        .cursor_world_pos(input.mouse_pixel_pos, &renderer.window.inner_size());
    let cursor_grid_pos = cursor_pos.to_grid(Point {
        x: state.world.width() as u32,
        y: state.world.width() as u32,
    });

    if input.just_pressed(Key::Escape) {
        return true;
    }
    if input.scroll_delta != 0.0 {
        state.camera_scroll += input.scroll_delta;
        state.camera.scale = (state.camera_scroll * 0.1).exp();
    }
    let move_dist = 0.2 / state.camera.scale;
    let pos = &mut state.camera.pos;

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

    if input.just_pressed(Key::V) {
        state.mouse_mode = match state.mouse_mode {
            MouseMode::Vel => MouseMode::Dens,
            MouseMode::Dens => MouseMode::Vel,
        }
    }

    if input.mouse_pressed(winit::event::MouseButton::Left) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.dens[i] += 1.0 * t_delta.as_secs_f32();
        }
    }
    if input.mouse_pressed(winit::event::MouseButton::Right) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.dens[i] = 0.0;
        }
    }
    if input.just_pressed(Key::Left) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.vx[i] = -1.0;
            state.world.vy[i] = 0.0;
            state.world.dens[i] += 5.0 * t_delta.as_secs_f32();
        }
    }
    if input.just_pressed(Key::Right) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.vx[i] = 1.0;
            state.world.vy[i] = 0.0;
            state.world.dens[i] += 5.0 * t_delta.as_secs_f32();
        }
    }
    if input.just_pressed(Key::Up) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.vy[i] = 1.0;
            state.world.vx[i] = 0.0;
            state.world.dens[i] += 5.0 * t_delta.as_secs_f32();
        }
    }
    if input.just_pressed(Key::Down) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.vy[i] = -1.0;
            state.world.vx[i] = 0.0;
            state.world.dens[i] += 5.0 * t_delta.as_secs_f32();
        }
    }
    if input.mouse_pressed(winit::event::MouseButton::Right) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.vx[i] = 0.0;
            state.world.vy[i] = 0.0;
            state.world.u_prev[i] = 0.0;
            state.world.v_prev[i] = 0.0;
        }
    }

    if input.just_pressed(Key::T) {
        println!("{}", state.world.dens.iter().sum::<f32>());
    }

    state.world.update(t_delta.as_secs_f32());
    for (i, dens) in state.world.dens.iter().enumerate() {
        match state.mouse_mode {
            MouseMode::Dens => {
                state.grid[i] = TileInstance {
                    r: 0.0,
                    g: 0.0,
                    b: *dens,
                    a: 0.0,
                }
            }
            MouseMode::Vel => {
                state.grid[i] = TileInstance {
                    r: state.world.vx[i],
                    g: state.world.vy[i],
                    b: -state.world.vx[i],
                    a: 0.0,
                }
            }
        }
    }

    false
}
