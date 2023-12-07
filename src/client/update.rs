use crate::world::water::C;

use super::{
    input::Input,
    render::{tile::TileInstance, Renderer},
    ClientState, MouseMode,
};
use std::time::Duration;
use winit::event::{MouseButton, VirtualKeyCode as Key};

pub fn update(
    state: &mut ClientState,
    input: &Input,
    renderer: &Renderer,
    t_delta: &Duration,
) -> bool {
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

    handle_water(state, input, renderer, t_delta);

    state.world.update(t_delta);

    for (i, dens) in state.world.water.rho.iter().enumerate() {
        if state.world.water.barrier[i] {
            state.grid[i].r = 1.0;
            state.grid[i].g = 1.0;
            state.grid[i].b = 1.0;
            state.grid[i].a = 1.0;
        } else {
            state.grid[i].r = *dens - 1.0;
            state.grid[i].g = 0.0;
            state.grid[i].b = *dens;
            state.grid[i].a = 1.0;
        }
    }

    false
}

pub fn handle_water(
    state: &mut ClientState,
    input: &Input,
    renderer: &Renderer,
    t_delta: &Duration,
) {
    let cursor_pos = state
        .camera
        .cursor_world_pos(input.mouse_pixel_pos, &renderer.window.inner_size());
    let cursor_grid_pos = cursor_pos.to_grid(state.world.size());

    if let Some(pos) = cursor_grid_pos {
        let i = pos.index(state.world.size().x);
        if input.mouse_pressed(MouseButton::Left) {
            state.world.water.lat[C][i] += 4.0 * t_delta.as_secs_f32();
        }
        if input.mouse_pressed(MouseButton::Right) {
            state.world.water.lat[C][i] = 0.0;
        }
        if input.pressed(Key::Left) {
            state.world.water.vel[i].x -= 1.0 * t_delta.as_secs_f32();
        }
        if input.pressed(Key::Right) {
            state.world.water.vel[i].x += 1.0 * t_delta.as_secs_f32();
        }
        if input.pressed(Key::Up) {
            state.world.water.vel[i].y += 1.0 * t_delta.as_secs_f32();
        }
        if input.pressed(Key::Down) {
            state.world.water.vel[i].y -= 1.0 * t_delta.as_secs_f32();
        }
        if input.just_pressed(Key::T) {
            println!("{}, {:?}", state.world.water.rho[i], state.world.water.vel[i])
        }
        if input.pressed(Key::B) {
            state.world.water.barrier[i] = true;
        }
    }
}
