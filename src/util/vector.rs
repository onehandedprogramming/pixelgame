use std::ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Default, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vec2<f32> {
    pub const CARDINAL_DIRECTIONS: [Self; 4] = [
        Self { x: 1.0, y: 0.0 },
        Self { x: 0.0, y: 1.0 },
        Self { x: -1.0, y: 0.0 },
        Self { x: 0.0, y: -1.0 },
    ];

    pub const X_UNIT: Self = Vec2 { x: 1.0, y: 0.0 };
    pub const Y_UNIT: Self = Vec2 { x: 0.0, y: 1.0 };

    pub fn dist(&self, other: Vec2<f32>) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }

    pub fn norm(self) -> Vec2<f32> {
        self / self.mag()
    }

    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    pub fn to_grid(&self, size: Vec2<usize>) -> Option<Vec2<usize>> {
        if self.x < 0.0 || self.y < 0.0 {
            return None;
        }
        let truncated = Vec2 {
            x: self.x as usize,
            y: self.y as usize,
        };
        if truncated.x > size.x - 1 || truncated.y > size.y - 1 {
            return None;
        }
        Some(truncated)
    }
}

impl Vec2<i32> {
    pub const CARDINAL_DIRECTIONS: [Self; 4] = [
        Self { x: 1, y: 0 },
        Self { x: 0, y: 1 },
        Self { x: -1, y: 0 },
        Self { x: 0, y: -1 },
    ];
    pub const CORNERS: [Self; 4] = [
        Self { x: -1, y: -1 },
        Self { x: -1, y: 1 },
        Self { x: 1, y: -1 },
        Self { x: 1, y: 1 },
    ];
}

impl<T: Mul<Output = T> + Copy> Vec2<T> {
    pub fn area(&self) -> T {
        self.x * self.y
    }
}

impl<T: Default + Copy> Vec2<T> {
    pub fn zero() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Vec2<T> {
    pub fn index(&self, width: T) -> T {
        self.y * width + self.x
    }
}

impl Vec2<i32> {
    pub fn clamp_usize(&self, max: Vec2<usize>) -> Vec2<usize> {
        return Vec2 {
            x: (self.x.max(0) as usize).min(max.x),
            y: (self.y.max(0) as usize).min(max.y),
        };
    }
}

impl<T: Neg<Output = T> + Copy> Neg for Vec2<T> {
    type Output = Vec2<T>;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Ord + Copy> Vec2<T> {
    pub fn min(&self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
    pub fn max(&self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl<T: Add<Output = T> + Copy> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Div<Output = T> + Copy> DivAssign for Vec2<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Into<Vec2<f32>> for Vec2<usize> {
    fn into(self) -> Vec2<f32> {
        return Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        };
    }
}

impl Into<Vec2<i32>> for Vec2<f32> {
    fn into(self) -> Vec2<i32> {
        return Vec2 {
            x: self.x as i32,
            y: self.y as i32,
        };
    }
}

impl Into<Vec2<u32>> for Vec2<i32> {
    fn into(self) -> Vec2<u32> {
        return Vec2 {
            x: self.x as u32,
            y: self.y as u32,
        };
    }
}

impl Into<Vec2<usize>> for Vec2<i32> {
    fn into(self) -> Vec2<usize> {
        return Vec2 {
            x: self.x as usize,
            y: self.y as usize,
        };
    }
}

impl Into<Vec2<usize>> for Vec2<f32> {
    fn into(self) -> Vec2<usize> {
        return Vec2 {
            x: self.x as usize,
            y: self.y as usize,
        };
    }
}

impl Into<Vec2<f32>> for Vec2<i32> {
    fn into(self) -> Vec2<f32> {
        return Vec2 {
            x: self.x as f32,
            y: self.y as f32,
        };
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T: BitAnd<Output = T> + Copy> BitAnd<T> for Vec2<T> {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self {
            x: self.x & rhs,
            y: self.y & rhs,
        }
    }
}
