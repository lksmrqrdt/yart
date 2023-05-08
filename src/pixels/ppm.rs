use crate::pixels::canvas::Canvas;

pub struct PPM {
    content: Vec<u8>,
}

impl PPM {
    pub fn new(canvas: &Canvas) -> Self {
        let mut header = PPM::create_header(canvas);
        let mut body = PPM::create_body(canvas);

        let mut content: Vec<u8> = Vec::with_capacity(header.len() + body.len());
        content.append(&mut header);
        content.append(&mut body);

        PPM { content }
    }

    fn create_header(canvas: &Canvas) -> Vec<u8> {
        format!("P6 {} {} 255\n", canvas.width(), canvas.height()).into_bytes()
    }

    fn create_body(canvas: &Canvas) -> Vec<u8> {
        let mut body: Vec<u8> = vec![0; 3 * canvas.width() * canvas.height()];

        for (i, color) in canvas.content().iter().enumerate() {
            let (r, g, b) = color.to_tuple();
            let offset = 3 * i;

            // Always set a block of all three hues at once.
            body[offset] = r;
            body[offset + 1] = g;
            body[offset + 2] = b;
        }

        body
    }

    pub fn get(&self) -> Vec<u8> {
        self.content.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixels::color::Color;

    #[test]
    fn new_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let header = PPM::create_header(&canvas);

        let desired_result = "P6 5 3 255\n".to_string().into_bytes();
        assert_eq!(desired_result, header);
    }

    #[test]
    fn new_ppm_body() {
        let mut canvas = Canvas::new(5, 3);
        let color_a = Color::new(1.5, 0.0, 0.0);
        let color_b = Color::new(0.0, 0.5, 0.0);
        let color_c = Color::new(-0.5, 0.0, 1.0);

        canvas.set_pixel(0, 0, color_a);
        canvas.set_pixel(2, 1, color_b);
        canvas.set_pixel(4, 2, color_c);

        let body = PPM::create_body(&canvas);

        for (count, _value) in body.iter().enumerate() {
            let desired_result = match count {
                0 | 44 => 255,
                22 => 127,
                _ => 0,
            };

            assert_eq!(desired_result, body[count]);
        }
    }

    #[test]
    fn new_ppm() {
        let mut canvas = Canvas::new(5, 3);
        let color_a = Color::new(1.5, 0.0, 0.0);
        let color_b = Color::new(0.0, 0.5, 0.0);
        let color_c = Color::new(-0.5, 0.0, 1.0);

        canvas.set_pixel(0, 0, color_a);
        canvas.set_pixel(2, 1, color_b);
        canvas.set_pixel(4, 2, color_c);

        let ppm = PPM::new(&canvas);

        let mut desired_result: Vec<u8> = Vec::new();
        desired_result.append(&mut PPM::create_header(&canvas));
        desired_result.append(&mut PPM::create_body(&canvas));

        assert_eq!(desired_result, ppm.get());
    }
}
