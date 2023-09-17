use crate::{point::Point, vector::Vector};

pub struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    pub const fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }

    pub const fn position(&self) -> Point {
        self.position
    }

    pub const fn velocity(&self) -> Vector {
        self.velocity
    }
}
