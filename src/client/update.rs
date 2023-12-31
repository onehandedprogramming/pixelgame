use std::time::Duration;

use crate::{
    client::{world::{W}, elements::{DEF_ELEMS, ElementType}},
    util::point::Point, get_element,
};

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
        x: W as u32,
        y: W as u32,
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
    if input.pressed(Key::S) {
        pos.y -= move_dist;
    }
    if input.pressed(Key::D) {
        pos.x += move_dist;
    }

    if input.just_pressed(Key::B) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(W as u32) as usize;
            state.world.cells.r[i] = get_element!(ElementType::Bendium);
        }
    }

    if input.just_pressed(Key::V) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(W as u32) as usize;
            state.world.cells.r[i] = get_element!(ElementType::Steam);
        }
    }

    if input.mouse_pressed(winit::event::MouseButton::Left) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(W as u32) as usize;
            state.world.cells.r[i] = get_element!(ElementType::Sand);
        }
    }
    if input.mouse_pressed(winit::event::MouseButton::Right) {
        if let Some(pos) = cursor_grid_pos {
            let i = pos.index(W as u32) as usize;
            state.world.cells.r[i] = get_element!(ElementType::Stone);
        }
        // if let Some(pos) = cursor_grid_pos {
        //     let i = pos.index(W as u32) as usize;
        //     state.world.cells.r[i] = Element {
        //         name: "Oil".into(),
        //         attributes: vec![Attribute::Fallable, Attribute::Liquid],
        //         color: ElementColor {
        //             r: 36.0 / 255.0,
        //             g: 32.0 / 255.0,
        //             b: 30.0 / 255.0,
        //         },
        //         heat: 0.0,
        //         moisture: 0.0,
        //         mass: 0.8,
        //     };
        // }
        // if let Some(pos) = cursor_grid_pos {
        //     let i = pos.index(W as u32) as usize;
        //     state.world.cells.r[i] = Element {
        //         name: "Dirt".into(),
        //         attributes: vec![Attribute::Fallable, Attribute::Solid],
        //         color: ElementColor {
                    // r: 115.0 / 255.0,
                    // g: 66.0 / 255.0,
                    // b: 33.0 / 255.0,
        //         },
        //         heat: 0.0,
        //         moisture: 0.0,
        //         mass: 8.5,
        //     };
        // }
    }
    // if input.just_pressed(Key::Left) {
    //     if let Some(pos) = cursor_grid_pos {
    //         let i = pos.index(state.world.width() as u32) as usize;
    //         state.world.nW[i] += 10.0;
    //     }
    // }
    // if input.just_pressed(Key::Right) {
    //     if let Some(pos) = cursor_grid_pos {
    //         let i = pos.index(state.world.width() as u32) as usize;
    //         state.world.ux[i] = 1.0;
    //         state.world.uy[i] = 0.0;
    //     }
    // }
    // if input.just_pressed(Key::Up) {
    //     if let Some(pos) = cursor_grid_pos {
    //         let i = pos.index(state.world.width() as u32) as usize;
    //         state.world.uy[i] = 1.0;
    //         state.world.ux[i] = 0.0;
    //     }
    // }
    // if input.just_pressed(Key::Down) {
    //     if let Some(pos) = cursor_grid_pos {
    //         let i = pos.index(state.world.width() as u32) as usize;
    //         state.world.uy[i] = -1.0;
    //         state.world.ux[i] = 0.0;
    //     }
    // }

    if input.just_pressed(Key::T) {
        // println!("all {}", state.world.cells..sum::<f32>());
        if let Some(pos) = cursor_grid_pos {
            println!(
                "cursor {:?}, pos: {:?}",
                state.world.cells.r[pos.y as usize * W + pos.x as usize],
                pos,
            );
        }
    }

    // if input.just_pressed(Key::C) || false {
    state.world.update(t_delta.as_secs_f32());
    // }

    // if let Some(pos) = cursor_grid_pos {
    //     if input.pressed(Key::B) {
    //         let i = pos.index(state.world.width() as u32) as usize;
    //         state.world.barrier[i] = true;
    //     }
    //     if input.pressed(Key::N) {
    //         let i = pos.index(state.world.width() as u32) as usize;
    //         state.world.barrier[i] = false;
    //     }
    // }
    fn convert_color(color: u32) -> (f32, f32, f32) {
        let red = ((color >> 16) & 0xFF) as f32 / 255.0;
        let green = ((color >> 8) & 0xFF) as f32 / 255.0;
        let blue = (color & 0xFF) as f32 / 255.0;

        (red, green, blue)
    }

    let mut buf = vec![0; super::world::H * super::world::W];
    state.world.render_to(&mut buf);
    for (i, col) in buf.iter().enumerate() {
        let color = convert_color(*col);
        state.grid[i] = TileInstance {
            r: color.0,
            g: color.1,
            b: color.2,
            a: 1.0,
        };
        // } else {
        //     match state.mouse_mode {
        //         MouseMode::Dens => TileInstance {
        //             r: 0.0,
        //             g: 0.0,
        //             b: (*dens - 0.08) * 10.0,
        //             a: 0.0,
        //         },
        //         MouseMode::Vel => TileInstance {
        //             r: (state.world.ux[i] * 5.0 + 1.0) / 2.0,
        //             g: (state.world.uy[i] * 5.0 + 1.0) / 2.0,
        //             b: 0.0,
        //             a: 0.0,
        //         }
        //     }
        // }
    }

    // if let Some(pos) = cursor_grid_pos {
    //     let i = pos.index(state.world.width() as u32) as usize;
    //     state.grid[i].r += 0.1;
    //     state.grid[i].g += 0.1;
    //     state.grid[i].b += 0.1;
    // }

    false
}
