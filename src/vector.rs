use std::ops::{Add, AddAssign};

use crate::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn from_points(lhs: Point, rhs: Point) -> Self {
        Self {
            x: lhs.x() - rhs.x(),
            y: lhs.y() - rhs.y(),
            z: lhs.z() - rhs.z(),
            w: lhs.w() - rhs.w(),
        }
    }

    pub const fn x(&self) -> f64 {
        self.x
    }

    pub const fn y(&self) -> f64 {
        self.y
    }

    pub const fn z(&self) -> f64 {
        self.z
    }

    pub const fn w(&self) -> f64 {
        self.w
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(mut self, other: Vector) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x();
        self.y += other.y();
        self.z += other.z();
        self.w += other.w();
    }
}
