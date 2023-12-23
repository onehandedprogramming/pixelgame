use super::{swap_buffer::SwapBuffer, elements::{WATER, AIR}};
use rand::Rng;

pub const W: usize = 100;
pub const H: usize = 100;

pub const EVAP_RATE: f32 = 0.01;
pub const CONDENS_RATE: f32 = 0.01;

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    CanEvaporate(Element),
    CanCondensate(Element),
    CanFall,
    Solid,
    Liquid,
    Gas,
    Immovable,
    Air,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ElementColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub name: Box<str>,
    pub attributes: Vec<Attribute>,
    pub color: ElementColor,
    pub heat: f32,
    pub moisture: f32,
    pub density: f32,
}

impl Element {
    fn render(&self) -> u32 {
        let r = (self.color.r * 255.0) as u32;
        let g = (self.color.g * 255.0) as u32;
        let b = (self.color.b * 255.0) as u32;

        (r << 16) | (g << 8) | b
    }
}

pub struct World {
    pub cells: SwapBuffer<Element>,
}

impl World {
    pub fn new() -> World {
        let mut rng = rand::thread_rng();

        let cells = (0..W * H)
            .map(|_| {
                if rng.gen::<bool>() {
                    WATER.clone()
                } else {
                    AIR.clone()
                }
            })
            .collect::<Vec<Element>>();

        World {
            cells: SwapBuffer::from_arr(cells, W),
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.cells.w.clone_from(&self.cells.r);

        // let er = &self.cells.r;
        let ew = &mut self.cells.w;

        let rng = &mut rand::thread_rng();

        update_main(rng, ew);
        update_gases(rng, ew);

        self.cells.swap();
    }

    pub fn render_to(&self, buf: &mut [u32]) {
        for i in 0..W {
            for j in 0..H {
                buf[j * W + i] = self.cells.r[j * W + i].render();
            }
        }
    }
}

fn update_main(rng: &mut rand::prelude::ThreadRng, ew: &mut Vec<Element>) {
    let (startx, endx, step) = if rng.gen::<bool>() {
        (0 as i32, W as i32, 1 as i32)
    } else {
        ((W - 1) as i32, -1, -1)
    };
    let mut ix = startx;

    while ix != endx {
        let x = ix as usize;

        let mut repeat_once = false;
        let mut y = 0;
        while y < H {
            let cell_index = y * W + x;
            let cell = &ew[y * W + x];
            if cell.attributes.contains(&Attribute::CanFall) {
                let positions_to_check = if rng.gen() {
                    [(0, -1), (-1, -1), (1, -1)]
                } else {
                    [(0, -1), (1, -1), (-1, -1)]
                };

                if let Some((dx, dy)) = positions_to_check.iter().find(|&&(dx, dy)| {
                    let new_x = (x as isize + dx) as usize;
                    let new_y = (y as isize + dy) as usize;

                    if !in_bounds(new_x as isize, new_y as isize) {
                        return false;
                    }

                    let other_cell = &ew[new_y * W + new_x];
                    let current_cell_immovable = ew[(y - 1) * W + x]
                        .attributes
                        .contains(&Attribute::Immovable);
                    let both_cells_solid = cell.attributes.contains(&Attribute::Solid)
                        && other_cell.attributes.contains(&Attribute::Solid);
                    let liquid_and_gas = cell.attributes.contains(&Attribute::Liquid)
                        && other_cell.attributes.contains(&Attribute::Gas);

                    !current_cell_immovable
                        && !liquid_and_gas
                        && (!both_cells_solid && other_cell.density < cell.density)
                }) {
                    let new_x = x as isize + dx;
                    let new_y = y as isize + dy;
                    ew.swap(cell_index, new_y as usize * W + new_x as usize);
                    if repeat_once {
                        repeat_once = false;
                        y += 1;
                    } else if ew[new_y as usize * W + new_x as usize]
                        .attributes
                        .contains(&Attribute::CanFall)
                    {
                        repeat_once = true;
                    }
                    continue;
                }
                if !cell.attributes.contains(&Attribute::Liquid) {
                    y += 1;
                    continue;
                }

                let positions_to_check = if rng.gen() {
                    [(-1, 0), (1, 0)]
                } else {
                    [(1, 0), (-1, 0)]
                };

                if let Some((dx, _)) = positions_to_check.iter().find(|&&(dx, _)| {
                    let new_x = (x as isize + dx) as usize;
                    let new_y = y;

                    if !in_bounds(new_x as isize, new_y as isize) {
                        return false;
                    }

                    let other_cell = &ew[new_y * W + new_x];

                    let liquid_and_gas = cell.attributes.contains(&Attribute::Liquid)
                        && other_cell.attributes.contains(&Attribute::Gas);

                    !liquid_and_gas && ew[new_y as usize * W + new_x as usize].density < cell.density
                }) {
                    let new_x = x as isize + dx;
                    ew.swap(cell_index, y * W + new_x as usize);
                    y += 1;
                    continue;
                }
            }

            repeat_once = false;
            y += 1;
        }
        ix = ix as i32 + step;
    }
}

fn update_gases(rng: &mut rand::prelude::ThreadRng, ew: &mut Vec<Element>) {
    let (startx, endx, step) = if rng.gen::<bool>() {
        (0 as i32, W as i32, 1 as i32)
    } else {
        ((W - 1) as i32, -1, -1)
    };
    let mut ix = startx;

    while ix != endx {
        let x = ix as usize;

        let mut repeat_once = false;
        let mut y = H - 1;
        while y >= 0 {
            let cell_index = y * W + x;
            let cell = &ew[cell_index];
            if cell.attributes.contains(&Attribute::Gas) {
                let mut positions_to_check = if rng.gen() {
                    vec![(0, 1), (-1, 1), (1, 1)]
                } else {
                    vec![(0, 1), (1, 1), (-1, 1)]
                };
                positions_to_check.extend(if rng.gen() {
                    [(-1, 0), (1, 0)]
                } else {
                    [(1, 0), (-1, 0)]
                });

                if let Some((dx, dy)) = positions_to_check.iter().find(|&&(dx, dy)| {
                    let new_x = (x as isize + dx) as usize;
                    let new_y = (y as isize + dy) as usize;

                    if !in_bounds(new_x as isize, new_y as isize) {
                        return false;
                    }

                    let other_cell = &ew[new_y * W + new_x];
                    let current_cell_immovable = new_y == y + 1
                        && ew[(y + 1) * W + x]
                            .attributes
                            .contains(&Attribute::Immovable);
                    let other_cell_fluid = other_cell.attributes.contains(&Attribute::Gas)
                        || other_cell.attributes.contains(&Attribute::Liquid) || other_cell.attributes.contains(&Attribute::Air);

                    !current_cell_immovable && (other_cell_fluid && other_cell.density > cell.density)
                }) {
                    let new_x = x as isize + dx;
                    let new_y = y as isize + dy;
                    ew.swap(cell_index, new_y as usize * W + new_x as usize);
                    if repeat_once {
                        repeat_once = false;
                        y -= 1;
                    } else if ew[new_y as usize * W + new_x as usize]
                        .attributes
                        .contains(&Attribute::Gas)
                    {
                        repeat_once = true;
                    }
                    continue;
                }
            }

            repeat_once = false;
            if y > 0 {
                y -= 1;
            } else {
                break;
            }
        }
        ix += step;
    }
}

fn in_bounds(x: isize, y: isize) -> bool {
    x >= 0 && x < W as isize && y >= 0 && y < H as isize
}
