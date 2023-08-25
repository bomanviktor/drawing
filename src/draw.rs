use super::geometrical_shapes::Drawable;
use super::{Circle, Line, Point, Rectangle, Triangle};
use raster::Image;

impl Drawable for Point {
    fn draw(&mut self, image: &mut Image) {
        if self.x >= 0 && self.x < image.width && self.y >= 0 && self.y < image.height {
            image
                .set_pixel(self.x as usize, self.y as usize, raster::Color::default())
                .unwrap();
        }
    }
}

impl Drawable for Line {
    fn draw(&mut self, image: &mut Image) {
        // Implementing Bresenham's Line Algorithm
        let mut x0 = self.point_a.x;
        let mut y0 = self.point_a.y;
        let x1 = self.point_b.x;
        let y1 = self.point_b.y;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            if x0 >= 0 && x0 < image.width && y0 >= 0 && y0 < image.height {
                image
                    .set_pixel(x0 as usize, y0 as usize, raster::Color::default())
                    .unwrap();
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

impl Drawable for Triangle {
    fn draw(&mut self, image: &mut Image) {
        // Drawing the three sides of the triangle
        Line {
            point_a: &self.point_a,
            point_b: &self.point_b,
        }
        .draw(image);
        Line {
            point_a: &self.point_b,
            point_b: &self.point_c,
        }
        .draw(image);
        Line {
            point_a: &self.point_a,
            point_b: &self.point_c,
        }
        .draw(image);
    }
}

impl Drawable for Rectangle {
    fn draw(&mut self, image: &mut Image) {
        // Drawing the four sides of the rectangle
        Line {
            point_a: &self.point_a,
            point_b: &self.point_b,
        }
        .draw(image);
        Line {
            point_a: &self.point_b,
            point_b: &self.point_c,
        }
        .draw(image);
        Line {
            point_a: &self.point_c,
            point_b: &self.point_d,
        }
        .draw(image);
        Line {
            point_a: &self.point_d,
            point_b: &self.point_a,
        }
        .draw(image);
    }
}

impl Drawable for Circle {
    fn draw(&mut self, image: &mut Image) {
        // Implementing Midpoint Circle Algorithm
        // ... (code for drawing the circle)

        // Note: This is a placeholder and needs a proper circle drawing algorithm.
    }
}
