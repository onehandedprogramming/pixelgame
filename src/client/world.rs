use super::swap_buffer::SwapBuffer;
use rand::Rng;

pub const W: usize = 100;
pub const H: usize = 100;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Attribute {
    Fallable,
    Solid,
    Liquid,
    Gas,
    Immovable,
}

#[derive(Clone, Copy, Debug)]
pub struct ElementColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Debug)]
pub struct Element {
    pub name: Box<str>,
    pub attributes: Vec<Attribute>,
    pub color: ElementColor,
    pub heat: f32,
    pub moisture: f32,
    pub mass: f32,
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
                    Element {
                        name: "Water".into(),
                        attributes: vec![Attribute::Fallable, Attribute::Liquid],
                        color: ElementColor {
                            r: 10.0 / 255.0,
                            g: 10.0 / 255.0,
                            b: 255.0 / 255.0,
                        },
                        heat: 0.0,
                        moisture: 0.0,
                        mass: 1.0,
                    }
                } else {
                    Element {
                        name: "Air".into(),
                        attributes: vec![],
                        color: ElementColor {
                            r: 80.0 / 255.0,
                            g: 180.0 / 255.0,
                            b: 210.0 / 255.0,
                        },
                        heat: 0.0,
                        moisture: 0.0,
                        mass: 0.05,
                    }
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
            if cell.attributes.contains(&Attribute::Fallable) {
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

                    !current_cell_immovable && (!both_cells_solid && other_cell.mass < cell.mass)
                }) {
                    let new_x = x as isize + dx;
                    let new_y = y as isize + dy;
                    ew.swap(cell_index, new_y as usize * W + new_x as usize);
                    if repeat_once {
                        repeat_once = false;
                        y += 1;
                    } else if ew[new_y as usize * W + new_x as usize]
                        .attributes
                        .contains(&Attribute::Fallable)
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
                    let new_x = x as isize + dx;
                    let new_y = y as isize;
                    in_bounds(new_x, new_y)
                        && ew[new_y as usize * W + new_x as usize].mass < cell.mass
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
                    let both_cells_solid = cell.attributes.contains(&Attribute::Solid)
                        && other_cell.attributes.contains(&Attribute::Solid);

                    !current_cell_immovable && (!both_cells_solid && other_cell.mass > cell.mass)
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
