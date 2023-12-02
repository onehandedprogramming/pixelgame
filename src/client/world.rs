const THE_N: usize = 98;
const SIZE: usize = (THE_N + 2) * (THE_N + 2);
pub struct World {
    pub u: Vec<f32>,
    pub v: Vec<f32>,
    pub dens: Vec<f32>,
    pub u_prev: Vec<f32>,
    pub v_prev: Vec<f32>,
    pub dens_prev: Vec<f32>,
}

impl World {
    pub fn new() -> Self {
        Self {
            u: vec![0.0; SIZE],
            v: vec![0.0; SIZE],
            dens: vec![0.0; SIZE],
            u_prev: vec![0.0; SIZE],
            v_prev: vec![0.0; SIZE],
            dens_prev: vec![0.0; SIZE],
        }
    }

    pub fn update(&mut self, dt: f32) {
        vel_step(
            THE_N,
            &mut self.u,
            &mut self.v,
            &mut self.u_prev,
            &mut self.v_prev,
            0.1,
            dt,
        );
        dens_step(
            THE_N,
            &mut self.dens,
            &mut self.dens_prev,
            &mut self.u,
            &mut self.v,
            0.01,
            dt,
        );
    }

    pub fn width(&self) -> usize {
        return THE_N + 2;
    }
}

fn add_source(n: usize, x: &mut [f32], s: &[f32], dt: f32) {
    for i in 0..(n + 2) * (n + 2) {
        x[i] += dt * s[i]
    }
}

fn ix(i: usize, j: usize) -> usize {
    i + (THE_N + 2) * j
}

fn diffuse(n: usize, b: i32, x: &mut [f32], x0: &[f32], diff: f32, dt: f32) {
    let a = dt * diff * (n * n) as f32;
    for _ in 0..20 {
        for i in 1..=n {
            for j in 1..=n {
                x[ix(i, j)] = (x0[ix(i, j)]
                    + a * (x[ix(i - 1, j)] + x[ix(i + 1, j)] + x[ix(i, j - 1)] + x[ix(i, j + 1)]))
                    / (1.0 + 4.0 * a);
            }
        }
        set_bnd(n, b, x);
    }
}

fn set_bnd(n: usize, b: i32, x: &mut [f32]) {
    for i in 1..=n {
        x[ix(0, i)] = if b == 1 { -x[ix(1, i)] } else { x[ix(1, i)] };
        x[ix(n + 1, i)] = if b == 1 { -x[ix(n, i)] } else { x[ix(n, i)] };
        x[ix(i, 0)] = if b == 2 { -x[ix(i, 1)] } else { x[ix(i, 1)] };
        x[ix(i, n + 1)] = if b == 2 { -x[ix(i, n)] } else { x[ix(i, n)] };
    }
    x[ix(0, 0)] = 0.5 * (x[ix(1, 0)] + x[ix(0, 1)]);
    x[ix(0, n + 1)] = 0.5 * (x[ix(1, n + 1)] + x[ix(0, n)]);
    x[ix(n + 1, 0)] = 0.5 * (x[ix(n, 0)] + x[ix(n + 1, 1)]);
    x[ix(n + 1, n + 1)] = 0.5 * (x[ix(n, n + 1)] + x[ix(n + 1, n)]);
}

fn advect(n: usize, b: i32, d: &mut [f32], d0: &[f32], u: &[f32], v: &[f32], dt: f32) {
    let dt0 = dt * n as f32;
    let n2 = n as f32;
    for i in 1..=n {
        for j in 1..=n {
            let mut x = i as f32 - dt0 * u[ix(i, j)];
            let mut y = j as f32 - dt0 * v[ix(i, j)];
            if x < 0.5 {
                x = 0.5;
            }
            if x > n2 + 0.5 {
                x = n2 + 0.5;
            };
            let i0 = x as usize;
            let i1 = i0 + 1;
            if y < 0.5 {
                y = 0.5;
            }
            if y > n2 + 0.5 {
                y = n2 + 0.5;
            };
            let j0 = y as usize;
            let j1 = j0 + 1;
            let s1 = x - i0 as f32;
            let s0 = 1.0 - s1;
            let t1 = y - j0 as f32;
            let t0 = 1.0 - t1;
            d[ix(i, j)] = s0 * (t0 * d0[ix(i0, j0)] + t1 * d0[ix(i0, j1)])
                + s1 * (t0 * d0[ix(i1, j0)] + t1 * d0[ix(i1, j1)]);
        }
    }
    set_bnd(n, b, d);
}

fn dens_step(
    n: usize,
    x: &mut Vec<f32>,
    x0: &mut Vec<f32>,
    u: &[f32],
    v: &[f32],
    diff: f32,
    dt: f32,
) {
    add_source(n, x, x0, dt);
    std::mem::swap(x, x0);
    diffuse(n, 0, x, x0, diff, dt);
    std::mem::swap(x, x0);
    advect(n, 0, x, x0, u, v, dt);
}

fn vel_step(
    n: usize,
    u: &mut Vec<f32>,
    v: &mut Vec<f32>,
    u0: &mut Vec<f32>,
    v0: &mut Vec<f32>,
    visc: f32,
    dt: f32,
) {
    add_source(n, u, u0, dt);
    add_source(n, v, v0, dt);
    std::mem::swap(u0, u);
    std::mem::swap(v0, v);
    diffuse(n, 1, u, u0, visc, dt);
    diffuse(n, 2, v, v0, visc, dt);
    project(n, u, v, u0, v0);
    std::mem::swap(u0, u);
    std::mem::swap(v0, v);
    advect(n, 1, u, u0, u0, v0, dt);
    advect(n, 2, v, v0, u0, v0, dt);
    project(n, u, v, u0, v0);
}

fn project(n: usize, u: &mut [f32], v: &mut [f32], p: &mut [f32], div: &mut [f32]) {
    let h = 1.0 / n as f32;
    for i in 1..=n {
        for j in 1..=n {
            div[ix(i, j)] =
                -0.5 * h * (u[ix(i + 1, j)] - u[ix(i - 1, j)] + v[ix(i, j + 1)] - v[ix(i, j - 1)]);
            p[ix(i, j)] = 0.0;
        }
    }

    set_bnd(n, 0, div);
    set_bnd(n, 0, p);

    for _ in 0..20 {
        for i in 1..=n {
            for j in 1..=n {
                p[ix(i, j)] = (div[ix(i, j)]
                    + p[ix(i - 1, j)]
                    + p[ix(i + 1, j)]
                    + p[ix(i, j - 1)]
                    + p[ix(i, j + 1)])
                    / 4.0;
            }
        }
        set_bnd(n, 0, p);
    }
    for i in 1..=n {
        for j in 1..=n {
            u[ix(i, j)] -= 0.5 * (p[ix(i + 1, j)] - p[ix(i - 1, j)]) / h;
            v[ix(i, j)] -= 0.5 * (p[ix(i, j + 1)] - p[ix(i, j - 1)]) / h;
        }
    }
    set_bnd(n, 1, u);
    set_bnd(n, 2, v);
}
