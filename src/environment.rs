use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    pub const fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }

    pub const fn gravity(&self) -> Vector {
        self.gravity
    }

    pub const fn wind(&self) -> Vector {
        self.wind
    }
}
