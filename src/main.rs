use raytracer_rs::pixels::canvas::Canvas;
use raytracer_rs::pixels::color::Color;
use raytracer_rs::pixels::ppm::PPM;
use raytracer_rs::tuples::coordinates::Coordinates;
use raytracer_rs::tuples::point::Point;
use raytracer_rs::tuples::scalar::Scalar;
use raytracer_rs::tuples::vector::Vector;

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
