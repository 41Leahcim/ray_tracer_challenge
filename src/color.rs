#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub const fn black() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub const fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    pub const fn red(&self) -> f64 {
        self.red
    }

    pub const fn green(&self) -> f64 {
        self.green
    }

    pub const fn blue(&self) -> f64 {
        self.blue
    }

    pub fn scale(&self, max: u32) -> [u32; 3] {
        let total_values = (max + 1) as f64; // Include 0 (0..=max is max + 1 values)
        [self.red, self.green, self.blue].map(|value| {
            let scaled = (value * total_values) as u32;
            scaled.clamp(0, max)
        })
    }
}
