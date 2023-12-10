use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub const fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub const fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn vector_from_points(lhs: Self, rhs: Self) -> Self {
        Self {
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z,
            w: lhs.w - rhs.w,
        }
    }
}

impl Add<Tuple> for Tuple {
    type Output = Self;

    fn add(mut self, other: Self) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign<Tuple> for Tuple {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}
