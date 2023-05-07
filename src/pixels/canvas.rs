use crate::pixels::color::Color;

#[derive(Clone, Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    content: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            content: vec![Color::default(); width * height],
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn content(&self) -> Vec<Color> {
        self.content.clone()
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let mapped_position = self.map_coordinates(x, y);
        self.content[mapped_position]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let mapped_position = self.map_coordinates(x, y);
        self.content[mapped_position] = color;
    }

    fn map_coordinates(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_canvas() {
        let canvas = Canvas::new(10, 20);

        let empty_pixel = &Color::default();

        assert_eq!(10, canvas.width());
        assert_eq!(20, canvas.height());

        for pixel in canvas.content().iter() {
            assert_eq!(empty_pixel, pixel);
        }
    }

    #[test]
    fn set_pixel_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red_pixel = Color::new(1.0, 0.0, 0.0);

        canvas.set_pixel(2, 3, red_pixel);

        assert_eq!(red_pixel, canvas.get_pixel(2, 3))
    }
}
