use crate::tuple::Tuple;

#[derive(Debug, Clone, Copy)]
pub struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Environment {
    pub const fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self { gravity, wind }
    }

    pub const fn gravity(&self) -> Tuple {
        self.gravity
    }

    pub const fn wind(&self) -> Tuple {
        self.wind
    }
}
