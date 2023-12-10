use crate::{color::Color, tuple::Tuple};

use std::{fmt::Write as _, fs::File, io::Write as _, path::Path};

pub const MAX_PPM_VALUE: u8 = 255;

#[derive(Debug, Clone)]
pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let capacity = width * height; // Capacity is known
        let pixels = (0..capacity).map(|_| Color::black()).collect::<Vec<_>>();
        Self {
            width,
            height,
            pixels,
        }
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }

    pub fn write_pixel(&mut self, point: &Tuple, value: Color) {
        let (x, y) = (point.x as u32, point.y as u32);
        assert!(x < self.width && y < self.height);
        self.pixels[(y * self.width + x) as usize] = value;
    }

    pub fn to_ppm(&self) -> String {
        self.pixels().iter().map(|color| color.scale(255)).fold(
            format!("P3\n{} {}\n{MAX_PPM_VALUE}", self.width(), self.height()),
            |mut output, color| {
                write!(output, "\n{} {} {}", color[0], color[1], color[2]).unwrap();
                output
            },
        )
    }

    pub fn save(&self, filename: impl AsRef<Path>) {
        let mut file = File::create(filename).unwrap();
        file.write_all(self.to_ppm().as_bytes()).unwrap();
    }
}
