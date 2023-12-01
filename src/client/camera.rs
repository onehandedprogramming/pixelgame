use winit::dpi::PhysicalSize;

use crate::util::point::Point;

const DEFAULT_ASPECT_RATIO: f32 = 16. / 9.;
const DEFAULT_SCALE: f32 = 0.02;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub pos: Point<f32>,
    pub aspect: f32,
    pub scale: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Point::zero(),
            aspect: DEFAULT_ASPECT_RATIO,
            scale: 1.0,
        }
    }
}

impl Camera {
    pub fn cursor_world_pos(&self, cursor_pos: Point<f32>, size: &PhysicalSize<u32>) -> Point<f32> {
        let p_size = Point {
            x: size.width as f32,
            y: size.height as f32,
        };
        let mut pos = cursor_pos / p_size * 2.0 - Point { x: 1.0, y: 1.0 };
        pos.y = -pos.y;
        pos = pos / self.proj_for(size);
        pos += self.pos;
        pos
    }

    pub fn proj_for(&self, size: &PhysicalSize<u32>) -> Point<f32> {
        let win_aspect = size.width as f32 / size.height as f32;
        let mut proj = if win_aspect > self.aspect {
            Point::new(1.0, win_aspect)
        } else {
            Point::new(self.aspect / win_aspect, self.aspect)
        };
        proj *= self.scale * DEFAULT_SCALE;
        proj
    }
}
