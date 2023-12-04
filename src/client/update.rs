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
            state.world.n0[i] += 20.0 * t_delta.as_secs_f32();
        }
    }
    if input.mouse_pressed(winit::event::MouseButton::Right) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.n0[i] = 0.0;
            // state.world.nN[i] = 0.0;
            // state.world.nS[i] = 0.0;
            // state.world.nE[i] = 0.0;
            // state.world.nW[i] = 0.0;
            // state.world.nNW[i] = 0.0;
            // state.world.nNE[i] = 0.0;
            // state.world.nSW[i] = 0.0;
            // state.world.nSE[i] = 0.0;
        }
    }
    if input.just_pressed(Key::Left) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.nW[i] += 10.0;
        }
    }
    if input.just_pressed(Key::Right) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.ux[i] = 1.0;
            state.world.uy[i] = 0.0;
        }
    }
    if input.just_pressed(Key::Up) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.uy[i] = 1.0;
            state.world.ux[i] = 0.0;
        }
    }
    if input.just_pressed(Key::Down) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.uy[i] = -1.0;
            state.world.ux[i] = 0.0;
        }
    }

    if input.just_pressed(Key::T) {
        println!("all {}", state.world.rho.iter().sum::<f32>());
        if let Some(pos) = cursor_grid_pos {
            println!("cursor {}", state.world.rho[pos.index(state.world.width() as u32) as usize]);
        }
    }

    // if input.just_pressed(Key::C) {
        state.world.update(t_delta.as_secs_f32());
    // }

    if let Some(pos) = cursor_grid_pos {
        if input.pressed(Key::B) {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.barrier[i] = true;
        }
        if input.pressed(Key::N) {
            let i = pos.index(state.world.width() as u32) as usize;
            state.world.barrier[i] = false;
        }
    }

    for (i, dens) in state.world.rho.iter().enumerate() {
        state.grid[i] = if state.world.barrier[i] {
            TileInstance {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }
        } else {
            match state.mouse_mode {
                MouseMode::Dens => TileInstance {
                    r: 0.0,
                    g: 0.0,
                    b: (*dens - 0.08) * 10.0,
                    a: 0.0,
                },
                MouseMode::Vel => TileInstance {
                    r: (state.world.ux[i] * 5.0 + 1.0) / 2.0,
                    g: (state.world.uy[i] * 5.0 + 1.0) / 2.0,
                    b: 0.0,
                    a: 0.0,
                }
            }
        }
    }

    if let Some(pos) = cursor_grid_pos {
        let i = pos.index(state.world.width() as u32) as usize;
        state.grid[i].r += 0.1;
        state.grid[i].g += 0.1;
        state.grid[i].b += 0.1;
    }

    false
}
