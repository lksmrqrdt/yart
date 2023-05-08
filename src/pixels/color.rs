use crate::pixels::rgb::Rgb;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn to_tuple(&self) -> (u8, u8, u8) {
        let r = into_u8(self.r());
        let g = into_u8(self.g());
        let b = into_u8(self.b());

        (r, g, b)
    }
}

fn into_u8(hue: f64) -> u8 {
    let converted = (hue * 255.0) as u8;
    converted.max(0).min(255)
}

impl Default for Color {
    fn default() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        (self.r() - other.r()).abs() < f64::EPSILON
            && (self.g() - other.g()).abs() < f64::EPSILON
            && (self.b() - other.b()).abs() < f64::EPSILON
    }
}

impl Rgb for Color {
    fn r(&self) -> f64 {
        self.r
    }

    fn g(&self) -> f64 {
        self.g
    }

    fn b(&self) -> f64 {
        self.b
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(self.r() - rhs.r(), self.g() - rhs.g(), self.b() - rhs.b())
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r() * rhs, self.g() * rhs, self.b() * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_color_constructor() {
        let color = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(-0.5, color.r());
        assert_eq!(0.4, color.g());
        assert_eq!(1.7, color.b());
    }

    #[test]
    fn default_color_constructor() {
        let color = Color::default();
        let zero_color = Color::new(0.0, 0.0, 0.0);

        assert_eq!(zero_color, color);
    }

    #[test]
    fn add_color() {
        let color_a = Color::new(0.9, 0.6, 0.75);
        let color_b = Color::new(0.7, 0.1, 0.25);

        let desired_result = Color::new(1.6, 0.7, 1.0);
        assert_eq!(desired_result, color_a + color_b);
    }

    #[test]
    fn sub_color() {
        let color_a = Color::new(0.9, 0.6, 0.75);
        let color_b = Color::new(0.7, 0.1, 0.25);

        let desired_result = Color::new(0.2, 0.5, 0.5);
        assert_eq!(desired_result, color_a - color_b);
    }

    #[test]
    fn mul_color() {
        let color_a = Color::new(1.0, 0.2, 0.4);
        let color_b = Color::new(0.9, 1.0, 0.1);

        let desired_result = Color::new(0.9, 0.2, 0.04);
        assert_eq!(desired_result, color_a * color_b);
    }

    #[test]
    fn mul_color_scalar() {
        let color = Color::new(0.2, 0.3, 0.4);

        let desired_result = Color::new(0.4, 0.6, 0.8);
        assert_eq!(desired_result, color * 2.0);
    }
}
