use vek::*;

pub const W: usize = 64;
pub const H: usize = 64;

#[derive(Copy, Clone)]
struct Square {
    p: Vec2<f32>,
    a: f32
}
impl Square {
    pub fn new(p0: Vec2<f32>, a0: f32) -> Self {
        Square {p: p0, a: a0}
    }

    pub fn shift(&mut self, offset: Vec2<f32>) {
        self.p += offset
    }

    pub fn grow(&mut self, amount: f32) {
        self.a += amount;
        self.p += Vec2::new(-0.5 * amount, -0.5 * amount);
    }
    
    pub fn intersect_area(&self, other: &Self) -> f32 {
        let x1 = self.p.x.max(other.p.x);
        let y1 = self.p.y.max(other.p.y);
        let x2 = (self.p.x + self.a).min(other.p.x + other.a);
        let y2 = (self.p.y + self.a).min(other.p.y + other.a);
        (x2 - x1).max(0.0) * (y2 - y1).max(0.0)
    }
}

#[derive(Copy, Clone)]
struct Cell {
    pop: f32,
    vel: Vec2<f32>,
    spread: f32,
    conductivity: f32,

}
impl Cell {
    pub fn empty() -> Self {
        Cell {
            pop: 0.0,
            vel: Vec2::zero(),
            spread: 0.0,
            conductivity: 1.0,
        }
    }

    pub fn set(&mut self, population: f32, velocity: Vec2<f32>, spread_factor: f32) {
        self.pop = population;
        self.vel = velocity;
        self.spread = spread_factor
    }

    pub fn clean(&mut self) {
        self.pop = 0.0;
    }
    
    #[inline(always)]
    pub fn flow_factor(&self, vec: Vec2<f32>, other: &Self, delta: f32) -> f32 {
        let pop_box = &mut Square::new(Vec2::zero(), 1.0);
        pop_box.grow(self.spread * delta);
        pop_box.shift(self.vel * delta);
        other.conductivity * pop_box.intersect_area(&Square::new(vec, 1.0))
    }
    
    #[inline(always)]
    fn update_vel(&mut self, new_vel: Vec2<f32>, pop_flow: f32) {
        if self.pop + pop_flow == 0.0 {
            self.vel = Vec2::zero();
            return
        }
        self.vel = (self.vel * self.pop + new_vel * pop_flow) / (self.pop + pop_flow)
    }

    #[inline(always)]
    fn update_spread(&mut self, new_spread: f32, pop_flow: f32) {
        if self.pop + pop_flow == 0.0 {
            self.spread = 0.0;
            return
        }
        self.spread = (self.spread * self.pop + new_spread * pop_flow) / (self.pop + pop_flow)
    }

    pub fn tick(&self, (this, left, right, up, down): (&mut Self, &mut Self, &mut Self, &mut Self, &mut Self), delta: f32) {
        /* save some effort quite often
        if self.pop == 0.0 {
            return
        }*/

        let flow_factors = [
            self.flow_factor(Vec2::zero(), this, delta),
            self.flow_factor(Vec2::left(), left, delta),
            self.flow_factor(Vec2::right(), right, delta),
            self.flow_factor(Vec2::up(), up, delta),
            self.flow_factor(Vec2::down(), down, delta),
        ];
        let flow_sum: f32 = (&flow_factors).iter().sum::<f32>();
        
        if flow_sum == 0.0 {
            return
        }
        let flow_vals = [
            self.pop * flow_factors[1] / flow_sum,
            self.pop * flow_factors[2] / flow_sum,
            self.pop * flow_factors[3] / flow_sum,
            self.pop * flow_factors[4] / flow_sum,
        ];
        let val_sum: f32 = (&flow_vals).iter().sum::<f32>();

        left.update_vel(self.vel, flow_vals[0]);
        right.update_vel(self.vel, flow_vals[1]);
        up.update_vel(self.vel, flow_vals[2]);
        down.update_vel(self.vel, flow_vals[3]);
        left.update_spread(self.spread, flow_vals[0]);
        right.update_spread(self.spread, flow_vals[1]);
        up.update_spread(self.spread, flow_vals[2]);
        down.update_spread(self.spread, flow_vals[3]);

        this.pop -= val_sum;
        left.pop += flow_vals[0];
        right.pop += flow_vals[1];
        up.pop += flow_vals[2];
        down.pop += flow_vals[3];

        this.vel.x += (left.pop - self.pop).powf(3.0) * 0.000000001;
        this.vel.x += (self.pop - right.pop).powf(3.0) * 0.000000001;
        this.vel.y -= (up.pop - self.pop).powf(3.0) * 0.000000001;
        this.vel.y -= (self.pop - down.pop).powf(3.0) * 0.000000001;
    }

    pub fn get_colour(&self) -> u32 {
        ((((1.0 - self.conductivity) * 255.0) as u32).min(255) << 16) + ((self.pop as u32).min(255)) as u32
    }
}


pub struct World {
    cells: Box<[[Cell; H]; W]>,
}

impl World {
    pub fn test(option: i32) -> Self {
        let mut this = Self {
            cells: Box::new([[Cell::empty(); H]; W]),
        };

        for i in 0..W {
            for j in 0..H {
                if
                    (20 + i as i32 - W as i32 / 2).wrapping_pow(2) +
                    (j as i32 - H as i32 / 2).wrapping_pow(2) < 100
                {
                    this.cells[i][j].set(250.0, Vec2::new(0.2, 0.1), 0.2);
                }
                if
                    (-20 + i as i32 - W as i32 / 2).wrapping_pow(2) +
                    (j as i32 - H as i32 / 2).wrapping_pow(2) < 100
                {
                    this.cells[i][j].set(250.0, Vec2::new(-0.2, 0.1), 0.2);
                }
            }
        }

        match option {
            2 => for i in 24..W - 24 {
                for j in 24..H - 24 {
                    this.cells[i][j].conductivity = (j as f32) / (H as f32)
                }
            },
            _ => {
                for i in 24..W - 24 {
                    this.cells[i][H / 3].conductivity = 0.0;
                }
                for i in 0..10 {
                    this.cells[20 + i][H / 6].conductivity = 0.0;
                    this.cells[W - 20 - i][H / 6].conductivity = 0.0;
                }
                for i in 0..W {
                    this.cells[i][0].conductivity = 0.0;
                    this.cells[i][0].pop = 400.0;
                    this.cells[i][H - 1].conductivity = 0.0;
                    this.cells[i][H - 1].pop = 400.0;
                }
                for j in 0..H {
                    this.cells[0][j].conductivity = 0.0;
                    this.cells[0][j].pop = 400.0;
                    this.cells[W - 1][j].conductivity = 0.0;
                    this.cells[W - 1][j].pop = 400.0;
                }
            },
        };

        this
    }

    pub fn update(&mut self, delta: f32) {
        let mut new_cells = self.cells.clone();
        for i in 1..W - 1 {
            for j in 1..H - 1 {
                let mut this = new_cells[i][j];
                let mut left = new_cells[i - 1][j];
                let mut right = new_cells[i + 1][j];
                let mut up = new_cells[i][j - 1];
                let mut down = new_cells[i][j + 1];
                self.cells[i][j].tick((
                    &mut this,
                    &mut left,
                    &mut right,
                    &mut up,
                    &mut down,
                ), delta);

                new_cells[i][j] = this;
                new_cells[i - 1][j] = left;
                new_cells[i + 1][j] = right;
                new_cells[i][j - 1] = up;
                new_cells[i][j + 1] = down;
            }
        }

        self.cells = new_cells;
    }

    pub fn render_to(&self, buf: &mut [u32]) {
        for i in 0..W {
            for j in 0..H {
                buf[j * W + i] = self.cells[i][j].get_colour();
            }
        }
    }
}
