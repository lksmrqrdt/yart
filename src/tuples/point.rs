use crate::tuples::coordinates::Coordinates;
use crate::tuples::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Point(f64, f64, f64);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point(x, y, z)
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::new(0.0, 0.0, 0.0)
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

impl std::ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.x(), -self.y(), -self.z())
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
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
        let point = Point::default();
        let zero_point = Point::new(0.0, 0.0, 0.0);

        assert_eq!(zero_point, point);
    }

    #[test]
    fn add_point_vector() {
        let point = Point::new(3.0, -2.0, 5.0);
        let vector = Vector::new(-2.0, 3.0, 1.0);

        let desired_result = Point::new(1.0, 1.0, 6.0);
        assert_eq!(desired_result, point + vector);
    }

    #[test]
    fn sub_point() {
        let point_a = Point::new(3.0, 2.0, 1.0);
        let point_b = Point::new(5.0, 6.0, 7.0);

        let desired_result = Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(desired_result, point_a - point_b);
    }

    #[test]
    fn sub_point_vector() {
        let point = Point::new(3.0, 2.0, 1.0);
        let vector = Vector::new(5.0, 6.0, 7.0);

        let desired_result = Point::new(-2.0, -4.0, -6.0);
        assert_eq!(desired_result, point - vector);
    }

    #[test]
    fn neg_point() {
        let point = Point::new(1.0, -2.0, 3.0);

        let desired_result = Point::new(-1.0, 2.0, -3.0);
        assert_eq!(desired_result, -point);
    }

    #[test]
    fn mul_point_scalar() {
        let point = Point::new(1.0, -2.0, 3.0);

        let desired_result = Point::new(3.5, -7.0, 10.5);
        assert_eq!(desired_result, point * 3.5);
    }

    #[test]
    fn mul_point_fraction() {
        let point = Point::new(1.0, -2.0, 3.0);

        let desired_result = Point::new(0.5, -1.0, 1.5);
        assert_eq!(desired_result, point * 0.5);
    }

    #[test]
    fn div_point() {
        let point = Point::new(1.0, -2.0, 3.0);

        let desired_result = Point::new(0.5, -1.0, 1.5);
        assert_eq!(desired_result, point / 2.0);
    }
}
