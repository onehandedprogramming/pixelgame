use std::time::Duration;

use crate::util::vector::Vec2;

use super::{smoke::SmokeGrid, liquid::LiquidGrid};

pub struct World {
    size: Vec2<usize>,
    pub smoke: SmokeGrid,
    pub water: LiquidGrid,
}

impl World {
    pub fn new() -> Self {
        Self {
            size: Vec2 { x: 100, y: 100 },
            smoke: SmokeGrid::new(100),
            water: LiquidGrid::new(Vec2 { x: 100, y: 100 }),
        }
    }
    pub fn update(&mut self, dt: &Duration) {
        // self.smoke.update(dt.as_secs_f32());
        self.water.update(dt.as_secs_f32());
    }
    pub fn size(&self) -> Vec2<usize> {
        self.size
    }
}
