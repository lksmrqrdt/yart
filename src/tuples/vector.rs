use crate::tuples::coordinates::Coordinates;
use crate::tuples::point::Point;

#[derive(Debug)]
pub struct Vector(f64, f64, f64);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector(x, y, z)
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.x() - other.x()).abs() < f64::EPSILON
            && (self.y() - other.y()).abs() < f64::EPSILON
            && (self.z() - other.z()).abs() < f64::EPSILON
    }
}

impl Coordinates for Vector {
    fn x(&self) -> f64 {
        self.0
    }

    fn y(&self) -> f64 {
        self.1
    }

    fn z(&self) -> f64 {
        self.2
    }
}

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_vector_constructor() {
        let vector = Vector(4.3, -4.2, 3.1);

        assert_eq!(4.3, vector.x());
        assert_eq!(-4.2, vector.y());
        assert_eq!(3.1, vector.z());
    }

    #[test]
    fn default_vector_constructor() {
        let default_vector = Vector::default();
        let custom_vector = Vector(0.0, 0.0, 0.0);

        assert_eq!(custom_vector, default_vector);
    }

    #[test]
    fn add_vector_point() {
        let vector = Vector::new(-2.0, 3.0, 1.0);
        let point = Point::new(3.0, -2.0, 5.0);

        let desired_result = Point::new(1.0, 1.0, 6.0);
        assert_eq!(desired_result, vector + point);
    }

    #[test]
    fn add_vector_vector() {
        let vector_a = Vector::new(-2.0, 3.0, 1.0);
        let vector_b = Vector::new(3.0, -2.0, 5.0);

        let desired_result = Vector::new(1.0, 1.0, 6.0);
        assert_eq!(desired_result, vector_a + vector_b);
    }
}
