use yart::pixels::canvas::Canvas;
use yart::pixels::color::Color;
use yart::pixels::ppm::PPM;
use yart::tuples::coordinates::Coordinates;
use yart::tuples::point::Point;
use yart::tuples::scalar::Scalar;
use yart::tuples::vector::Vector;

#[derive(Clone, Copy, Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Clone, Copy, Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn main() {
    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(1.0, 0.0, 0.0);

    let position = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut projectile = Projectile { position, velocity };

    let gravity = Vector::new(0.0, -0.1, 0.0);
    let wind = Vector::new(-0.01, 0.0, 0.0);
    let environment = Environment { gravity, wind };

    while projectile.position.y() > 0.0 {
        projectile = tick(environment, projectile);
        canvas.set_pixel(
            projectile.position.x() as usize,
            canvas.height()
                - (projectile.position.y() as usize)
                    .max(0)
                    .min(canvas.height())
                - 1,
            color,
        );
    }

    let ppm = PPM::new(&canvas);

    std::fs::write("output.ppm", ppm.get()).unwrap();
}

fn tick(environment: Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;

    Projectile { position, velocity }
}
