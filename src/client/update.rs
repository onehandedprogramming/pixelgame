use super::{input::Input, ClientState, render::tile::TileInstance};
use winit::event::VirtualKeyCode as Key;

pub fn update(state: &mut ClientState, input: &Input) -> bool {
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
    if input.pressed(Key::T) {
        state.grid[0] = TileInstance {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }

    false
}
