use crate::util::point::Point;

use super::{input::Input, ClientState, render::{tile::TileInstance, Renderer}};
use winit::event::VirtualKeyCode as Key;

pub fn update(state: &mut ClientState, input: &Input, renderer: &Renderer) -> bool {
    let cursor_pos = state.camera.cursor_world_pos(input.mouse_pixel_pos, &renderer.window.inner_size());
    let cursor_grid_pos = cursor_pos.to_grid(Point {x: 100, y: 100});

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
    if input.mouse_pressed(winit::event::MouseButton::Left) {
        if let Some(pos) = cursor_grid_pos {
            state.grid[pos.index(100) as usize] = TileInstance {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }
        }
    }
    if input.mouse_pressed(winit::event::MouseButton::Right) {
        if let Some(pos) = cursor_grid_pos {
            state.grid[pos.index(100) as usize] = TileInstance {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }
        }
    }

    false
}
