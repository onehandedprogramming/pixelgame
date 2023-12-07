use crate::util::{swap_buf::SwapBuffer, vector::Vec2};

pub struct LiquidGrid {
    pub size: Vec2<usize>,
    pub pos: SwapBuffer<Vec2<f32>>,
    pub vel: SwapBuffer<Vec2<f32>>,
    pub dens: SwapBuffer<f32>,
    pub barrier: Vec<bool>,
}

impl LiquidGrid {
    pub fn new(size: Vec2<usize>) -> Self {
        let len = size.area();
        let mut barrier = vec![false; len];
        for x in 0..2 {
            let x = x * (size.x - 1);
            for y in 0..size.y {
                barrier[x + y * size.x] = true;
            }
        }
        for y in 0..2 {
            let y = y * (size.y - 1);
            for x in 0..size.x {
                barrier[x + y * size.x] = true;
            }
        }
        Self {
            size,
            pos: SwapBuffer::new(len),
            vel: SwapBuffer::new(len),
            dens: SwapBuffer::new(len),
            barrier,
        }
    }

    pub fn update(&mut self, dt: f32) {
        for (d, v) in self.dens.iter().zip(self.vel.iter_mut()) {
            if *d > 0.0 {
                v.y -= 100.0 * dt;
            }
        }
        let (dr, dw, dm) = self.dens.rwm();
        let (vr, vw, vm) = self.vel.rwm();
        let (pr, pw, pm) = self.pos.rwm();
        for y in 1..self.size.y - 1 {
            let y_add = y * self.size.x;
            for x in 1..self.size.x - 1 {
                let i = x + y_add;
                let mut den = dr[i];
                if den < 0.001 {
                    den = 0.0;
                }
                if den == 0.0 {
                    pw[i] = Vec2::zero();
                    dw[i] = den;
                    vw[i] = Vec2::zero();
                    continue;
                }
                let mut pressure = (den - 0.2).max(0.0);
                let mut vel = vr[i];
                let mut pos = pr[i];
                pos.x = pos.x.clamp(-0.5, 0.5);
                pos.y = pos.y.clamp(-0.5, 0.5);

                let mut new_pos = pos + vel * dt;
                new_pos.x = new_pos.x.clamp(-1.49, 1.49);
                new_pos.y = new_pos.y.clamp(-1.49, 1.49);
                let mut change = new_pos - pos;
                // TODO i32 to isize
                let ipos: Vec2<i32> = (pos + change + 0.5).floor().into();
                if ipos.x != 0 || ipos.y != 0 {
                    let ix = (i as i32 + ipos.x) as usize;
                    let iy = (i as i32 + ipos.y * self.size.x as i32) as usize;
                    let i2 = (iy as i32 + ipos.x) as usize;
                    if self.barrier[i2] || self.barrier[ix] || self.barrier[iy] {
                        let mut mult = Vec2 {
                            x: (!self.barrier[ix]) as i32 as f32,
                            y: (!self.barrier[iy]) as i32 as f32,
                        };
                        if self.barrier[i2] && mult == (Vec2 { x: 1.0, y: 1.0 }) {
                            mult = Vec2::zero();
                        }
                        pressure += (vel * (Vec2 { x: 1.0, y: 1.0 } - mult)).mag() * den;
                        change = change * mult;
                        vel = vel * mult;
                    }
                }

                if pressure > 0.0 {
                    let mut moved = 0.0;
                    for (dirs, mult) in [
                        (Vec2::<i32>::CARDINAL_DIRECTIONS, 1.0 / 9.0),
                        (Vec2::<i32>::CORNERS, 1.0 / 36.0),
                    ] {
                        for dir in dirs {
                            let i2 = (i as i32 + dir.x + dir.y * self.size.x as i32) as usize;
                            if !self.barrier[i2] && dr[i2] < den {
                                let change =
                                    (mult * (den - dr[i2])).min(den * mult) * dt * 100.0;
                                dm[i2] += change;

                                let fdir: Vec2<f32> = dir.into();
                                let fdir = fdir * 0.1 + vel * change;
                                let sum = change + dr[i2];
                                vm[i2] += (fdir * change + vr[i2] * dr[i2]) / sum - vr[i2];

                                moved += change;
                            }
                        }
                    }
                    den -= moved;
                }

                pos += change;
                let ipos: Vec2<i32> = (pos + 0.5).floor().into();
                if ipos.x != 0 || ipos.y != 0 {
                    let i2 = (i as i32 + ipos.x + ipos.y * self.size.x as i32) as usize;
                    let move_den = (den + vel.mag() * 0.1 - dr[i2] * 2.0).max(0.0).min(den);
                    let sum = move_den + dr[i2];
                    let mov: Vec2<f32> = ipos.into();
                    let new_pos = pos - mov;
                    pm[i2] += (new_pos * move_den + pr[i2] * dr[i2]) / sum - pr[i2];
                    vm[i2] += (vel * move_den + vr[i2] * dr[i2]) / sum - vr[i2];
                    dm[i2] += move_den;

                    den -= move_den;
                    pos = Vec2::zero();
                    vel = Vec2::zero();
                }
                pw[i] = pos;
                dw[i] = den;
                vw[i] = vel;
            }
        }
        self.dens.swap();
        self.vel.swap();
        self.pos.swap();
    }
}
