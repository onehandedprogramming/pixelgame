use crate::util::vector::Vec2;

pub const SW: usize = 0;
pub const S: usize = 1;
pub const SE: usize = 2;
pub const W: usize = 3;
pub const C: usize = 4;
pub const E: usize = 5;
pub const NW: usize = 6;
pub const N: usize = 7;
pub const NE: usize = 8;

pub struct WaterGrid {
    pub size: Vec2<usize>,
    pub lat: [Vec<f32>; 9],
    pub pos: Vec<Vec2<f32>>,
    pub vel: Vec<Vec2<f32>>,
    pub pos_move: Vec<Vec2<f32>>,
    pub vel_move: Vec<Vec2<f32>>,
    pub rho: Vec<f32>,
    pub ux: Vec<f32>,
    pub uy: Vec<f32>,
    pub curl: Vec<f32>,
    pub barrier: Vec<bool>,
}

impl WaterGrid {
    pub fn new(size: Vec2<usize>) -> Self {
        let len = size.area();
        let mut barrier = vec![false; len];
        for x in 0..2 {
            let x = x * (size.x - 3) + 1;
            for y in 1..size.y - 1 {
                barrier[x + y * size.x] = true;
            }
        }
        for y in 0..2 {
            let y = y * (size.y - 3) + 1;
            for x in 1..size.x - 1 {
                barrier[x + y * size.x] = true;
            }
        }
        Self {
            size,
            lat: std::array::from_fn(|_| vec![0.0; len]),
            pos: vec![Vec2::zero(); len],
            vel: vec![Vec2::zero(); len],
            pos_move: vec![Vec2::zero(); len],
            vel_move: vec![Vec2::zero(); len],
            rho: vec![0.0; len],
            ux: vec![0.0; len],
            uy: vec![0.0; len],
            curl: vec![0.0; len],
            barrier,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.collide(dt);
        self.stream();
    }

    fn collide(&mut self, dt: f32) {
        for y in 1..self.size.y - 1 {
            for x in 1..self.size.x - 1 {
                let i = x + y * self.size.x;
                let dens = self.lat[C][i];
                if dens > 0.0 {
                    self.vel[i].y -= 100.0 * dt;
                    self.vel[i].x -= 100.0 * dt;
                } else {
                    self.vel[i] = Vec2::zero();
                }
                self.rho[i] = dens;
                self.pos[i] += self.vel[i] * dt;
                if dens > 1.0 {
                    let mut add = 0.0;
                    let val = self.lat[C][i] * 1.0 / 9.0;
                    for Vec2 { x, y } in Vec2::<i32>::CARDINAL_DIRECTIONS {
                        let i2 = (i as i32 + x + y * self.size.x as i32) as usize;
                        let li = (x + 1 + (y + 1) * 3) as usize;
                        if self.barrier[i2] {
                            add += val;
                        } else {
                            self.lat[li][i] += val;
                        }
                    }
                    let val = self.lat[C][i] * 1.0 / 36.0;
                    for Vec2 { x, y } in Vec2::<i32>::CORNERS {
                        let i2 = (i as i32 + x + y * self.size.x as i32) as usize;
                        let li = (x + 1 + (y + 1) * 3) as usize;
                        if self.barrier[i2] {
                            add += val;
                        } else {
                            self.lat[li][i] += val;
                        }
                    }
                    self.pos_move[i] = Vec2::zero();
                    self.lat[C][i] *= 4.0 / 9.0;
                    self.lat[C][i] += add;
                }
                let dx = ((self.pos[i].x * 2.0) as i32).min(1).max(-1);
                let dy = ((self.pos[i].y * 2.0) as i32).min(1).max(-1);
                if dx != 0 || dy != 0 {
                    let i2 = (i as i32 + dx + dy * self.size.x as i32) as usize;
                    let diff_vec = Into::<Vec2<f32>>::into(Vec2 { x: dx, y: dy });
                    if self.barrier[i2] {
                        if dx != 0 && dy != 0 {
                            let i2x = (i as i32 + dx) as usize;
                            let i2y = (i as i32 + dy * self.size.x as i32) as usize;
                            if self.barrier[i2x] {
                                self.vel[i].x *= -dx.abs() as f32 * 0.9;
                            }
                            if self.barrier[i2y] {
                                self.vel[i].y *= -dx.abs() as f32 * 0.9;
                            }
                        } else {
                            self.vel[i] = self.vel[i] * (-diff_vec.abs() * 0.9);
                        }
                    } else {
                        let li = (dx + 1 + (dy + 1) * 3) as usize;
                        self.lat[li][i] += self.lat[C][i];
                        self.lat[C][i] = 0.0;
                        self.pos_move[i] = self.pos[i] - diff_vec;
                        self.pos[i] = Vec2::zero();
                        self.vel_move[i] = self.vel[i];
                        self.vel[i] = Vec2::zero();
                    }
                }
                // let still_dens = self.lat[C][i];
            }
        }
    }

    fn stream(&mut self) {
        for y in (1..self.size.y - 1).rev() {
            for x in 1..self.size.x - 1 {
                let i = x + y * self.size.x;
                let i2 = i - self.size.x;
                let tot = self.lat[C][i] + self.lat[N][i2];
                if tot != 0.0 {
                    self.pos[i] =
                        (self.pos[i] * self.lat[C][i] + self.pos_move[i2] * self.lat[N][i2]) / tot;
                    let vel_move = if self.vel_move[i2] == Vec2::zero() {
                        Vec2 {x: 0.0, y: 0.1}
                    } else {
                        self.vel_move[i2]
                    };
                    self.vel[i] =
                        (self.vel[i] * self.lat[C][i] + vel_move * self.lat[N][i2]) / tot;
                }
                self.lat[C][i] += self.lat[N][i2];
                self.lat[N][i2] = 0.0;

                let i2 = i + 1 - self.size.x;
                let tot = self.lat[C][i] + self.lat[NW][i2];
                if tot != 0.0 {
                    self.pos[i] =
                        (self.pos[i] * self.lat[C][i] + self.pos_move[i2] * self.lat[NW][i2]) / tot;
                    let vel_move = if self.vel_move[i2] == Vec2::zero() {
                        Vec2 {x: -0.1, y: 0.1}
                    } else {
                        self.vel_move[i2]
                    };
                    self.vel[i] =
                        (self.vel[i] * self.lat[C][i] + vel_move * self.lat[NW][i2]) / tot;
                }
                self.lat[C][i] += self.lat[NW][i2];
                self.lat[NW][i2] = 0.0;
            }
        }
        for y in (1..self.size.y - 1).rev() {
            for x in (1..self.size.x - 1).rev() {
                let i = x + y * self.size.x;
                let i2 = i - 1;
                let tot = self.lat[C][i] + self.lat[E][i2];
                if tot != 0.0 {
                    self.pos[i] =
                        (self.pos[i] * self.lat[C][i] + self.pos_move[i2] * self.lat[E][i2]) / tot;
                    let vel_move = if self.vel_move[i2] == Vec2::zero() {
                        Vec2 {x: 0.1, y: 0.0}
                    } else {
                        self.vel_move[i2]
                    };
                    self.vel[i] =
                        (self.vel[i] * self.lat[C][i] + vel_move * self.lat[E][i2]) / tot;
                }
                self.lat[C][i] += self.lat[E][i2];
                self.lat[E][i2] = 0.0;

                let i2 = i - 1 - self.size.x;
                let tot = self.lat[C][i] + self.lat[NE][i2];
                if tot != 0.0 {
                    self.pos[i] =
                        (self.pos[i] * self.lat[C][i] + self.pos_move[i2] * self.lat[NE][i2]) / tot;
                    let vel_move = if self.vel_move[i2] == Vec2::zero() {
                        Vec2 {x: 0.1, y: 0.1}
                    } else {
                        self.vel_move[i2]
                    };
                    self.vel[i] =
                        (self.vel[i] * self.lat[C][i] + vel_move * self.lat[NE][i2]) / tot;
                }
                self.lat[C][i] += self.lat[NE][i2];
                self.lat[NE][i2] = 0.0;
            }
        }
        for y in 1..self.size.y - 1 {
            for x in (1..self.size.x - 1).rev() {
                let i = x + y * self.size.x;
                let i2 = i + self.size.x;
                let tot = self.lat[C][i] + self.lat[S][i2];
                if tot != 0.0 {
                    self.pos[i] =
                        (self.pos[i] * self.lat[C][i] + self.pos_move[i2] * self.lat[S][i2]) / tot;
                    let vel_move = if self.vel_move[i2] == Vec2::zero() {
                        Vec2 {x: 0.0, y: -0.1}
                    } else {
                        self.vel_move[i2]
                    };
                    self.vel[i] =
                        (self.vel[i] * self.lat[C][i] + vel_move * self.lat[S][i2]) / tot;
                }
                self.lat[C][i] += self.lat[S][i2];
                self.lat[S][i2] = 0.0;

                let i2 = i - 1 + self.size.x;
                let tot = self.lat[C][i] + self.lat[SE][i2];
                if tot != 0.0 {
                    self.pos[i] =
                        (self.pos[i] * self.lat[C][i] + self.pos_move[i2] * self.lat[SE][i2]) / tot;
                    let vel_move = if self.vel_move[i2] == Vec2::zero() {
                        Vec2 {x: 0.1, y: -0.1}
                    } else {
                        self.vel_move[i2]
                    };
                    self.vel[i] =
                        (self.vel[i] * self.lat[C][i] + vel_move * self.lat[SE][i2]) / tot;
                }
                self.lat[C][i] += self.lat[SE][i2];
                self.lat[SE][i2] = 0.0;
            }
        }
        for y in 1..self.size.y - 1 {
            for x in 1..self.size.x - 1 {
                let i = x + y * self.size.x;
                let i2 = i + 1;
                let tot = self.lat[C][i] + self.lat[W][i2];
                if tot != 0.0 {
                    self.pos[i] =
                        (self.pos[i] * self.lat[C][i] + self.pos_move[i2] * self.lat[W][i2]) / tot;
                    let vel_move = if self.vel_move[i2] == Vec2::zero() {
                        Vec2 {x: -0.1, y: 0.0}
                    } else {
                        self.vel_move[i2]
                    };
                    self.vel[i] =
                        (self.vel[i] * self.lat[C][i] + vel_move * self.lat[W][i2]) / tot;
                }
                self.lat[C][i] += self.lat[W][i2];
                self.lat[W][i2] = 0.0;

                let i2 = i + 1 + self.size.x;
                let tot = self.lat[C][i] + self.lat[SW][i2];
                if tot != 0.0 {
                    self.pos[i] =
                        (self.pos[i] * self.lat[C][i] + self.pos_move[i2] * self.lat[SW][i2]) / tot;
                    let vel_move = if self.vel_move[i2] == Vec2::zero() {
                        Vec2 {x: -0.1, y: -0.1}
                    } else {
                        self.vel_move[i2]
                    };
                    self.vel[i] =
                        (self.vel[i] * self.lat[C][i] + vel_move * self.lat[SW][i2]) / tot;
                }
                self.lat[C][i] += self.lat[SW][i2];
                self.lat[SW][i2] = 0.0;
            }
        }
    }
}
