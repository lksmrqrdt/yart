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
    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    while projectile.position.y() > 0.0 {
        projectile = tick(environment, projectile);
        dbg!(projectile);
    }
}

fn tick(environment: Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;

    Projectile { position, velocity }
}
