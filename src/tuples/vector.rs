use crate::tuples::coordinates::Coordinates;
use crate::tuples::point::Point;
use crate::tuples::scalar::Scalar;

#[derive(Clone, Copy, Debug)]
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

impl Scalar for Vector {
    type Output = Vector;

    fn magnitude(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    fn normalize(&self) -> Self::Output {
        let magnitude = self.magnitude();

        Vector::new(
            self.x() / magnitude,
            self.y() / magnitude,
            self.z() / magnitude,
        )
    }

    fn dot_product(&self, rhs: Vector) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    fn cross_product(&self, rhs: Self) -> Self::Output {
        Vector::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x(), -self.y(), -self.z())
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
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
    fn add_vector() {
        let vector_a = Vector::new(-2.0, 3.0, 1.0);
        let vector_b = Vector::new(3.0, -2.0, 5.0);

        let desired_result = Vector::new(1.0, 1.0, 6.0);
        assert_eq!(desired_result, vector_a + vector_b);
    }

    #[test]
    fn add_vector_point() {
        let vector = Vector::new(-2.0, 3.0, 1.0);
        let point = Point::new(3.0, -2.0, 5.0);

        let desired_result = Point::new(1.0, 1.0, 6.0);
        assert_eq!(desired_result, vector + point);
    }

    #[test]
    fn sub_vector() {
        let vector_a = Vector::new(3.0, 2.0, 1.0);
        let vector_b = Vector::new(5.0, 6.0, 7.0);

        let desired_result = Vector::new(-2.0, -4.0, -6.0);

        assert_eq!(desired_result, vector_a - vector_b);
    }

    #[test]
    fn sub_vector_zero() {
        let vector = Vector::new(1.0, -2.0, 3.0);
        let zero = Vector::new(0.0, 0.0, 0.0);

        let desired_result = Vector::new(-1.0, 2.0, -3.0);
        assert_eq!(desired_result, zero - vector);
    }

    #[test]
    fn neg_vector() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let desired_result = Vector::new(-1.0, 2.0, -3.0);
        assert_eq!(desired_result, -vector);
    }

    #[test]
    fn mul_vector_scalar() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let desired_result = Vector::new(3.5, -7.0, 10.5);
        assert_eq!(desired_result, vector * 3.5);
    }

    #[test]
    fn mul_vector_fraction() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let desired_result = Vector::new(0.5, -1.0, 1.5);
        assert_eq!(desired_result, vector * 0.5);
    }

    #[test]
    fn div_vector() {
        let vector = Vector::new(1.0, -2.0, 3.0);

        let desired_result = Vector::new(0.5, -1.0, 1.5);
        assert_eq!(desired_result, vector / 2.0);
    }

    #[test]
    fn magnitude_vector_x() {
        let vector = Vector::new(1.0, 0.0, 0.0);

        let desired_result = 1.0;
        assert_eq!(desired_result, vector.magnitude());
    }

    #[test]
    fn magnitude_vector_y() {
        let vector = Vector::new(0.0, 1.0, 0.0);

        let desired_result = 1.0;
        assert_eq!(desired_result, vector.magnitude());
    }

    #[test]
    fn magnitude_vector_z() {
        let vector = Vector::new(0.0, 0.0, 1.0);

        let desired_result = 1.0;
        assert_eq!(desired_result, vector.magnitude());
    }

    #[test]
    fn magnitude_vector_positive() {
        let vector = Vector::new(1.0, 2.0, 3.0);

        let desired_result = f64::sqrt(14.0);
        assert_eq!(desired_result, vector.magnitude());
    }

    #[test]
    fn magnitude_vector_negative() {
        let vector = Vector::new(-1.0, -2.0, -3.0);

        let desired_result = f64::sqrt(14.0);
        assert_eq!(desired_result, vector.magnitude());
    }

    #[test]
    fn normalize_vector_x() {
        let vector = Vector::new(4.0, 0.0, 0.0);

        let desired_result = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(desired_result, vector.normalize());
    }

    #[test]
    fn normalize_vector_all() {
        let vector = Vector::new(1.0, 2.0, 3.0);

        let magnitude = f64::sqrt(14.0);
        let desired_result = Vector::new(1.0 / magnitude, 2.0 / magnitude, 3.0 / magnitude);
        assert_eq!(desired_result, vector.normalize());
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let vector = Vector::new(1.0, 2.0, 3.0);
        let normalized = vector.normalize();

        let desired_result = 1.0;
        assert_eq!(desired_result, normalized.magnitude())
    }

    #[test]
    fn dot_product_vector() {
        let vector_a = Vector::new(1.0, 2.0, 3.0);
        let vector_b = Vector::new(2.0, 3.0, 4.0);

        let desired_result = 20.0;
        assert_eq!(desired_result, vector_a.dot_product(vector_b))
    }

    #[test]
    fn cross_product_vector() {
        let vector_a = Vector::new(1.0, 2.0, 3.0);
        let vector_b = Vector::new(2.0, 3.0, 4.0);

        let desired_result = Vector::new(-1.0, 2.0, -1.0);
        assert_eq!(desired_result, vector_a.cross_product(vector_b));
        assert_eq!(-desired_result, vector_b.cross_product(vector_a));
    }
}
