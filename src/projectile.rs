use crate::tuple::Tuple;

pub struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    pub const fn new(position: Tuple, velocity: Tuple) -> Self {
        Self { position, velocity }
    }

    pub const fn position(&self) -> Tuple {
        self.position
    }

    pub const fn velocity(&self) -> Tuple {
        self.velocity
    }
}
