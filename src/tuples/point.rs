use crate::tuples::coordinates::Coordinates;
use crate::tuples::vector::Vector;

#[derive(Debug)]
pub struct Point(f64, f64, f64);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point(x, y, z)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x() - other.x()).abs() < f64::EPSILON
            && (self.y() - other.y()).abs() < f64::EPSILON
            && (self.z() - other.z()).abs() < f64::EPSILON
    }
}

impl Coordinates for Point {
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

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_point_constructor() {
        let point = Point::new(4.3, -4.2, 3.1);

        assert_eq!(4.3, point.x());
        assert_eq!(-4.2, point.y());
        assert_eq!(3.1, point.z());
    }

    #[test]
    fn default_point_constructor() {
        let default_point = Point::default();
        let custom_point = Point::new(0.0, 0.0, 0.0);

        assert_eq!(custom_point, default_point);
    }

    #[test]
    fn add_point_vector() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vector = Vector::new(-2.0, 3.0, 1.0);

        let desired_result = Point::new(1.0, 1.0, 6.0);
        assert_eq!(desired_result, point + vector);
    }
}
