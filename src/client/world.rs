const VISC: f32 = 0.2;

pub struct World {
    pub xdim: usize,
    pub ydim: usize,
    pub n0: Vec<f32>,			// microscopic densities along each lattice direction
	pub nN: Vec<f32>,
	pub nS: Vec<f32>,
	pub nE: Vec<f32>,
	pub nW: Vec<f32>,
	pub nNE: Vec<f32>,
    pub nSE: Vec<f32>,
	pub nNW: Vec<f32>,
	pub nSW: Vec<f32>,
	pub rho: Vec<f32>,			// macroscopic density
	pub ux: Vec<f32>,			// macroscopic velocity
	pub uy: Vec<f32>,
	pub curl: Vec<f32>,
	pub barrier: Vec<bool>,	
}

impl World {
    pub fn new(dimx: usize, dimy: usize) -> Self {
        let size = dimx * dimy;
        let mut s = Self {
            xdim: 128,
            ydim: 128,
            n0: vec![0.0; size],
            nN: vec![0.0; size],
            nS: vec![0.0; size],
            nE: vec![0.0; size],
            nW: vec![0.0; size],
            nNE: vec![0.0; size],
            nSE: vec![0.0; size],
            nNW: vec![0.0; size],
            nSW: vec![0.0; size],
            rho: vec![0.0; size],
            ux: vec![0.0; size],
            uy: vec![0.0; size],
            curl: vec![0.0; size],
            barrier: vec![false; size],
        };
        s.initFluid();
        s
    }

    pub fn update(&mut self, dt: f32) {
        self.collide();
        self.stream();
    }

    pub fn width(&self) -> usize {
        return self.xdim;
    }

    // Collide particles within each cell (here's the physics!):
    fn collide(&mut self) {
        let viscosity = VISC;	// kinematic viscosity coefficient in natural units
        let omega = 1.0 / (3.0*viscosity + 0.5);		// reciprocal of relaxation time
        for y in 1..self.ydim-1 {
            for x in 1..self.xdim-1 {
                let i = x + y*self.xdim;		// array index for this lattice site
                let thisrho = self.n0[i] + self.nN[i] + self.nS[i] + self.nE[i] + self.nW[i]
                    + self.nNW[i] + self.nNE[i] + self.nSW[i] + self.nSE[i];
                self.rho[i] = thisrho;
                if thisrho <= 0.0 {
                    continue;
                }
                let thisux = (self.nE[i] + self.nNE[i] + self.nSE[i] - self.nW[i] - self.nNW[i] - self.nSW[i]) / thisrho;
                self.ux[i] = thisux;
                let thisuy = (self.nN[i] + self.nNE[i] + self.nNW[i] - self.nS[i] - self.nSE[i] - self.nSW[i]) / thisrho;
                self.uy[i] = thisuy;
                let one9thrho = (1.0/9.0) * thisrho;		// pre-compute a bunch of stuff for optimization
                let one36thrho = (1.0/36.0) * thisrho;
                let ux3 = 3.0 * thisux;
                let uy3 = 3.0 * thisuy;
                let ux2 = thisux * thisux;
                let uy2 = thisuy * thisuy;
                let uxuy2 = 2.0 * thisux * thisuy;
                let u2 = ux2 + uy2;
                let u215 = 1.5 * u2;
                self.n0[i]  += omega * ((4.0/9.0)*thisrho * (1.0                         - u215) - self.n0[i]);
                self.nE[i]  += omega * (   one9thrho * (1.0 + ux3       + 4.5*ux2        - u215) - self.nE[i]);
                self.nW[i]  += omega * (   one9thrho * (1.0 - ux3       + 4.5*ux2        - u215) - self.nW[i]);
                self.nN[i]  += omega * (   one9thrho * (1.0 + uy3       + 4.5*uy2        - u215) - self.nN[i]);
                self.nS[i]  += omega * (   one9thrho * (1.0 - uy3       + 4.5*uy2        - u215) - self.nS[i]);
                self.nNE[i] += omega * (  one36thrho * (1.0 + ux3 + uy3 + 4.5*(u2+uxuy2) - u215) - self.nNE[i]);
                self.nSE[i] += omega * (  one36thrho * (1.0 + ux3 - uy3 + 4.5*(u2-uxuy2) - u215) - self.nSE[i]);
                self.nNW[i] += omega * (  one36thrho * (1.0 - ux3 + uy3 + 4.5*(u2-uxuy2) - u215) - self.nNW[i]);
                self.nSW[i] += omega * (  one36thrho * (1.0 - ux3 - uy3 + 4.5*(u2+uxuy2) - u215) - self.nSW[i]);
            }
        }
        // for y in 1..self.ydim-2 {
        //     self.nW[self.xdim-1+y*self.xdim] = self.nW[self.xdim-2+y*self.xdim];		// at right end, copy left-flowing densities from next row to the left
        //     self.nNW[self.xdim-1+y*self.xdim] = self.nNW[self.xdim-2+y*self.xdim];
        //     self.nSW[self.xdim-1+y*self.xdim] = self.nSW[self.xdim-2+y*self.xdim];
        // }
    }

    // Move particles along their directions of motion:
    fn stream(&mut self) {
        let mut barrierCount = 0;
        let mut barrierxSum = 0;
        let mut barrierySum = 0;
        let mut barrierFx = 0.0;
        let mut barrierFy = 0.0;

        for y in (1..self.ydim-1).rev() {		// first start in NW corner...
            for x in 1..self.xdim-1 {
                self.nN[x+y*self.xdim] = self.nN[x+(y-1)*self.xdim];			// move the north-moving particles
                self.nNW[x+y*self.xdim] = self.nNW[x+1+(y-1)*self.xdim];		// and the northwest-moving particles
            }
        }
        for y in (1..self.ydim-1).rev() { 		// now start in NE corner...
            for x in (1..self.xdim-1).rev() {
                self.nE[x+y*self.xdim] = self.nE[x-1+y*self.xdim];			// move the east-moving particles
                self.nNE[x+y*self.xdim] = self.nNE[x-1+(y-1)*self.xdim];		// and the northeast-moving particles
            }
        }
        for y in 1..self.ydim-1 {			// now start in SE corner...
            for x in (1..self.xdim-1).rev() {
                self.nS[x+y*self.xdim] = self.nS[x+(y+1)*self.xdim];			// move the south-moving particles
                self.nSE[x+y*self.xdim] = self.nSE[x-1+(y+1)*self.xdim];		// and the southeast-moving particles
            }
        }
        for y in 1..self.ydim-1 {                		// now start in the SW corner...
            for x in 1..self.xdim-1 {
                self.nW[x+y*self.xdim] = self.nW[x+1+y*self.xdim];			// move the west-moving particles
                self.nSW[x+y*self.xdim] = self.nSW[x+1+(y+1)*self.xdim];		// and the southwest-moving particles
            }
        }
        for y in 1..self.ydim-1 {               		// Now handle bounce-back from barriers
            for x in 1..self.xdim-1 {
                if self.barrier[x+y*self.xdim] {
                    let index = x + y*self.xdim;
                    self.nE[x+1+y*self.xdim] = self.nW[index];
                    self.nW[x-1+y*self.xdim] = self.nE[index];
                    self.nN[x+(y+1)*self.xdim] = self.nS[index];
                    self.nS[x+(y-1)*self.xdim] = self.nN[index];
                    self.nNE[x+1+(y+1)*self.xdim] = self.nSW[index];
                    self.nNW[x-1+(y+1)*self.xdim] = self.nSE[index];
                    self.nSE[x+1+(y-1)*self.xdim] = self.nNW[index];
                    self.nSW[x-1+(y-1)*self.xdim] = self.nNE[index];
                    // Keep track of stuff needed to plot force vector:
                    barrierCount += 1;
                    barrierxSum += x;
                    barrierySum += y;
                    barrierFx += self.nE[index] + self.nNE[index] + self.nSE[index] - self.nW[index] - self.nNW[index] - self.nSW[index];
                    barrierFy += self.nN[index] + self.nNE[index] + self.nNW[index] - self.nS[index] - self.nSE[index] - self.nSW[index];
                }
            }
        }
    }

    fn initFluid(&mut self) {
		for y in 0..self.ydim {
			for x in 0..self.xdim {
				self.setEquil(x, y, 0.1, 0.0, 0.1);
			}
		}
	}

    fn setEquil(&mut self, x: usize, y: usize, newux: f32, newuy: f32, newrho: f32) {
		let i = x + y*self.xdim;
		let ux3 = 3.0 * newux;
		let uy3 = 3.0 * newuy;
		let ux2 = newux * newux;
		let uy2 = newuy * newuy;
		let uxuy2 = 2.0 * newux * newuy;
		let u2 = ux2 + uy2;
		let u215 = 1.5 * u2;
		self.n0[i]  =  (4.0/9.0) * newrho * (1.0                              - u215);
		self.nE[i]  =  (1.0/9.0) * newrho * (1.0 + ux3       + 4.5*ux2        - u215);
		self.nW[i]  =  (1.0/9.0) * newrho * (1.0 - ux3       + 4.5*ux2        - u215);
		self.nN[i]  =  (1.0/9.0) * newrho * (1.0 + uy3       + 4.5*uy2        - u215);
		self.nS[i]  =  (1.0/9.0) * newrho * (1.0 - uy3       + 4.5*uy2        - u215);
		self.nNE[i] = (1.0/36.0) * newrho * (1.0 + ux3 + uy3 + 4.5*(u2+uxuy2) - u215);
		self.nSE[i] = (1.0/36.0) * newrho * (1.0 + ux3 - uy3 + 4.5*(u2-uxuy2) - u215);
		self.nNW[i] = (1.0/36.0) * newrho * (1.0 - ux3 + uy3 + 4.5*(u2-uxuy2) - u215);
		self.nSW[i] = (1.0/36.0) * newrho * (1.0 - ux3 - uy3 + 4.5*(u2+uxuy2) - u215);
		self.rho[i] = newrho;
		self.ux[i] = newux;
		self.uy[i] = newuy;
	}
}