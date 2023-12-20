pub struct SwapBuffer<T> {
    width: usize,
    pub r: Vec<T>,
    pub w: Vec<T>,
}

impl<T: Sync + Send + Clone> SwapBuffer<T> {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.r, &mut self.w);
    }
    pub fn swap_cells(&mut self, pos1: usize, pos2: usize) {
        self.r.swap(pos1, pos2);
    }
    pub fn swap_cells_w(&mut self, pos1: usize, pos2: usize) {
        self.w.swap(pos1, pos2);
    }
}

impl<T: Clone> SwapBuffer<T> {
    pub fn from_arr(base: Vec<T>, width: usize) -> SwapBuffer<T> {
        SwapBuffer {
            width,
            r: base.clone(),
            w: base,
        }
    }
}

