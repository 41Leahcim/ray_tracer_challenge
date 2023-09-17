use std::ops::{Add, AddAssign};

use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64, // X-axis
    y: f64, // Y-axis
    z: f64, // Z-axis
    w: f64, // Special value denoting a point
}

impl Point {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
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

impl Add<Vector> for Point {
    type Output = Self;

    fn add(mut self, other: Vector) -> Self::Output {
        self += other;
        self
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x();
        self.y += other.y();
        self.z += other.z();
        self.w += other.w();
    }
}
