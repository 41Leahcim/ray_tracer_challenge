use std::{fs::File, io::Write};

use color::Color;
use environment::Environment;
use point::Point;
use projectile::Projectile;
use vector::Vector;

use crate::canvas::Canvas;

mod canvas;
mod color;
mod environment;
mod point;
mod projectile;
mod vector;

fn tick(env: &Environment, projectile: Projectile) -> Projectile {
    Projectile::new(
        projectile.position() + projectile.velocity(),
        projectile.velocity() + env.gravity() + env.wind(),
    )
}

fn main() {
    let gravity = Vector::new(0.0, -0.1, 0.0); // Negative Y to push down
    let wind = Vector::new(-0.01, 0.0, 0.0); // Negative X to add resistance
    let env = Environment::new(gravity, wind);

    // Magic numbers, AKA machine learning params
    let position = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(4.0, 7.2, 0.0);
    let mut projctile = Projectile::new(position, velocity);

    let mut canvas = Canvas::new(500, 300);

    let blue = Color::new(0.0, 0.0, 1.0);
    assert_eq!(blue.scale(255), [0, 0, 255]);

    while projctile.position().y() > 0.0 {
        // Not heavy enough to bore into the earth
        projctile = tick(&env, projctile);

        // Draw the current position of the projectile
        let position = projctile.position();
        if position.y() < 0.0 {
            break;
        }
        let pos_y = canvas.height() - (position.y() as u32); // Flip y
        if pos_y < canvas.height() && position.x() >= 0.0 && (position.x() as u32) < canvas.width()
        {
            canvas.write_pixel(position.x() as u32, pos_y, blue);
        }
    }
    let ppm = canvas.to_ppm();
    println!("{ppm}");
    let mut file = File::create("images/trajectory.ppm").unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
