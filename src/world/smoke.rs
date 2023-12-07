pub struct SmokeGrid {
    inner_size: usize,
    pub vx: Vec<f32>,
    pub vy: Vec<f32>,
    pub dens: Vec<f32>,
    pub u_prev: Vec<f32>,
    pub v_prev: Vec<f32>,
    pub dens_prev: Vec<f32>,
}

impl SmokeGrid {
    pub fn new(size: usize) -> Self {
        let area = size * size;
        Self {
            inner_size: size - 2,
            vx: vec![0.0; area],
            vy: vec![0.0; area],
            dens: vec![0.0; area],
            u_prev: vec![0.0; area],
            v_prev: vec![0.0; area],
            dens_prev: vec![0.0; area],
        }
    }

    pub fn update(&mut self, dt: f32) {
        vel_step(
            self.inner_size,
            &mut self.vx,
            &mut self.vy,
            &mut self.u_prev,
            &mut self.v_prev,
            0.000001,
            dt,
        );
        dens_step(
            self.inner_size,
            &mut self.dens,
            &mut self.dens_prev,
            &mut self.vx,
            &mut self.vy,
            0.01,
            dt,
        );
    }
}

fn add_source(x: &mut [f32], s: &mut [f32], dt: f32) {
    for i in 0..x.len() {
        x[i] += dt * s[i];
    }
}

fn ix(i: usize, j: usize) -> usize {
    i + 100 * j
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

fn advect(n: usize, b: i32, d: &mut [f32], d0: &[f32], vx: &[f32], vy: &[f32], dt: f32) {
    let n2 = n as f32;
    let dt0 = dt * n as f32;
    for i in 1..=n {
        for j in 1..=n {
            let mut x = i as f32 - dt0 * vx[ix(i, j)];
            let mut y = j as f32 - dt0 * vy[ix(i, j)];
            // println!("x={}", dt0 * vx[ix(i, j)]);
            // println!("y={}", dt0 * vy[ix(i, j)]);
            if x < 0.5 {
                x = 0.5;
            }
            if x > n2 + 0.5 {
                x = n2 + 0.5;
            };
            if y < 0.5 {
                y = 0.5;
            }
            if y > n2 + 0.5 {
                y = n2 + 0.5;
            };

            let i0 = x.floor() as usize;
            let i1 = i0 + 1;
            let j0 = y.floor() as usize;
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
    d: &mut Vec<f32>,
    d0: &mut Vec<f32>,
    vx: &[f32],
    vy: &[f32],
    diff: f32,
    dt: f32,
) {
    // std::mem::swap(d, d0);
    // diffuse(n, 0, d, d0, diff, dt);
    std::mem::swap(d, d0);
    advect(n, 0, d, d0, vx, vy, dt);
}

fn vel_step(
    n: usize,
    vx: &mut Vec<f32>,
    vy: &mut Vec<f32>,
    vx0: &mut Vec<f32>,
    vy0: &mut Vec<f32>,
    visc: f32,
    dt: f32,
) {
    add_source(vx, vx0, dt);
    add_source(vy, vy0, dt);
    std::mem::swap(vx0, vx);
    std::mem::swap(vy0, vy);
    diffuse(n, 1, vx, vx0, visc, dt);
    diffuse(n, 2, vy, vy0, visc, dt);
    project(n, vx, vy, vx0, vy0);
    std::mem::swap(vx0, vx);
    std::mem::swap(vy0, vy);
    advect(n, 1, vx, vx0, vx0, vy0, dt);
    advect(n, 2, vy, vy0, vx0, vy0, dt);
    project(n, vx, vy, vx0, vy0);
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
