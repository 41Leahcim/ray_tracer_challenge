use std::f64::consts::PI;

use color::Color;
use matrix::Matrix;
use tuple::Tuple;

use crate::canvas::Canvas;

mod canvas;
mod color;
mod matrix;
mod tuple;

const WIDTH: u32 = 8000;
const HEIGHT: u32 = 8000;
const POINTS_IN_A_CIRCLE: u32 = if WIDTH < HEIGHT { WIDTH } else { HEIGHT } * 4;

/// 2 PI radians in a circle, 12 hours on a clock
const RADIANS_IN_AN_HOUR: f64 = 2.0 * PI / 12.0;
const RADIANS_IN_A_MINUTE: f64 = RADIANS_IN_AN_HOUR / 5.0;
const RADIANS_IN_A_DEGREE: f64 = 2.0 * PI / 360.0;
const RADIANS_IN_A_CIRCLE: f64 = 2.0 * PI / POINTS_IN_A_CIRCLE as f64;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let start_points = Tuple::point(0.0, -(WIDTH as f64) / (2.0 + 1.0 / WIDTH as f64), 0.0);

    for point in 0..POINTS_IN_A_CIRCLE {
        let transformation = Matrix::identity()
            .rotate_z(point as f64 * RADIANS_IN_A_CIRCLE)
            .translate((canvas.width() / 2) as f64, (HEIGHT / 2) as f64, 0.0);
        let new_point = &transformation * start_points;
        canvas.write_pixel(&new_point, Color::white());
    }

    canvas.save("images/clock.ppm");
}
