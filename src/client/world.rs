use super::swap_buffer::SwapBuffer;
use rand::Rng;

pub const W: usize = 100;
pub const H: usize = 100;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Attribute {
    Fallable,
    Liquid,
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
        let er = &self.cells.r;
        let ew = &mut self.cells.w;

        for x in 0..W {
            for y in 0..H {
                ew[y * W + x] = er[y * W + x].clone();
            }
        }

        let mut rng = rand::thread_rng();

        let (startx, endx, step) = if rng.gen::<bool>() {
            (0 as i32, W as i32, 1 as i32)
        } else {
            ((W - 1) as i32, -1, -1)
        };
        let mut ix = startx;

        while ix != endx {
            let x = ix as usize;
            for y in (0..H).rev() {
                let cell_index = y * W + x;
                let cell = &er[y * W + x];
                if cell.attributes.contains(&Attribute::Fallable) {
                    let positions_to_check = if rng.gen() {
                        [(0, -1), (-1, -1), (1, -1)]
                    } else {
                        [(0, -1), (1, -1), (-1, -1)]
                    };

                    if let Some((dx, dy)) = positions_to_check.iter().find(|&&(dx, dy)| {
                        let new_x = x as isize + dx;
                        let new_y = y as isize + dy;
                        in_bounds(new_x, new_y) && er[new_y as usize * W + new_x as usize].mass < cell.mass
                    }) {
                        let new_x = x as isize + dx;
                        let new_y = y as isize + dy;
                        ew.swap(cell_index, new_y as usize * W + new_x as usize);
                        continue;
                    }
                    if !cell.attributes.contains(&Attribute::Liquid) {
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
                        in_bounds(new_x, new_y) && er[new_y as usize * W + new_x as usize].mass < cell.mass
                    }) {
                        let new_x = x as isize + dx;
                        ew.swap(cell_index, y * W + new_x as usize);
                    }
                }
            }
            ix = ix as i32 + step;
        }

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

fn in_bounds(x: isize, y: isize) -> bool {
    x >= 0 && x < W as isize && y >= 0 && y < H as isize
}
