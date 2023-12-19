use super::swap_buffer::SwapBuffer;
use rand::Rng;

pub const W: usize = 100;
pub const H: usize = 100;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ElementType {
    Air,
    Sand,
    Water,
    Fire,
}

#[derive(Clone, Copy, Debug)]
pub struct ElementColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub element_type: ElementType,
    pub color: ElementColor,
    pub heat: f32,
    pub moisture: f32,
    pub mass: f32,
}

impl Cell {
    fn render(&self) -> u32 {
        let r = (self.color.r * 255.0) as u32;
        let g = (self.color.g * 255.0) as u32;
        let b = (self.color.b * 255.0) as u32;

        (r << 16) | (g << 8) | b
    }
}

pub struct World {
    pub cells: SwapBuffer<Cell>,
}

impl World {
    pub fn new() -> World {
        let mut rng = rand::thread_rng();

        let cells = (0..W * H)
            .map(|_| {
                if rng.gen::<bool>() {
                    Cell {
                        element_type: ElementType::Water,
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
                    Cell {
                        element_type: ElementType::Air,
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
            .collect::<Vec<Cell>>();

        World {
            cells: SwapBuffer::from_arr(cells, W),
        }
    }

    pub fn update(&mut self, delta: f32) {
        let er = &self.cells.r;
        let ew = &mut self.cells.w;

        for x in 0..W {
            for y in 0..H {
                ew[y * W + x] = er[y * W + x];
            }
        }

        let mut rng = rand::thread_rng();

        let (startx, endx, step) = if rng.gen::<bool>() {
            (0 as i32,W as i32,1 as i32)
        } else {
            ((W-1) as i32,-1,-1)
        };
        let mut ix = startx;

        while ix != endx {
            let x = ix as usize;
            for y in (0..H).rev() {
                let cell = er[y * W + x];
                if cell.element_type == ElementType::Sand {
                    let cell = er[y * W + x];
                    if cell.element_type == ElementType::Sand {
                        let mut positions_to_check = vec![(0, -1)];

                        if rng.gen() {
                            positions_to_check.push((-1, -1));
                            positions_to_check.push((1, -1));
                        } else {
                            positions_to_check.push((1, -1));
                            positions_to_check.push((-1, -1));
                        }

                        for (dx, dy) in positions_to_check {
                            let new_x = x as isize + dx;
                            let new_y = y as isize + dy;

                            if in_bounds(new_x, new_y)
                                && er[new_y as usize * W + new_x as usize].mass < cell.mass
                            {
                                ew.swap(y * W + x, new_y as usize * W + new_x as usize);
                                break;
                            }
                        }
                    }
                } else if cell.element_type == ElementType::Water {
                    let mut rng = rand::thread_rng();
                    let mut positions_to_check = vec![(0, -1)];

                    if rng.gen() {
                        positions_to_check.push((-1, -1));
                        positions_to_check.push((1, -1));
                    } else {
                        positions_to_check.push((1, -1));
                        positions_to_check.push((-1, -1));
                    }

                    // if x == 15 && y == 5 {
                    //     println!("Below: {:?}", er[(y - 1) * W + x]);
                    // }

                    let mut moved = false;
                    for (dx, dy) in positions_to_check {
                        let new_x = x as isize + dx;
                        let new_y = y as isize + dy;

                        if in_bounds(new_x, new_y)
                            && er[new_y as usize * W + new_x as usize].mass < cell.mass
                        {
                            ew.swap(y * W + x, new_y as usize * W + new_x as usize);
                            moved = true;
                            break;
                        }
                    }
                    if moved {
                        continue;
                    }

                    let mut positions_to_check = vec![];
                    if rng.gen() {
                        positions_to_check.push((-1, 0));
                        positions_to_check.push((1, 0));
                    } else {
                        positions_to_check.push((1, 0));
                        positions_to_check.push((-1, 0));
                    }

                    for (dx, dy) in positions_to_check {
                        let new_x = x as isize + dx;
                        let new_y = y as isize + dy;

                        if in_bounds(new_x, new_y)
                            && er[new_y as usize * W + new_x as usize].element_type
                                == ElementType::Air
                        {
                            ew.swap(y * W + x, new_y as usize * W + new_x as usize);
                            break;
                        }
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
