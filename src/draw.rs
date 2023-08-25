use super::geometrical_shapes::{Circle, Drawable, Line, Point, Rectangle, Triangle};
use raster::Image;

impl Drawable for Point {
    fn draw(&mut self, image: &mut Image) {
        if self.x >= 0 && self.x < image.width && self.y >= 0 && self.y < image.height {
            image
                .set_pixel(self.x, self.y, raster::Color::rgb(255, 255, 255))
                .unwrap();
        }
    }

    fn color(&mut self) {
        todo!()
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
                    .set_pixel(
                        (x0 as usize).try_into().unwrap(),
                        (y0 as usize).try_into().unwrap(),
                        raster::Color::rgb(255, 255, 255),
                    )
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

    fn color(&mut self) {
        todo!()
    }
}

impl Drawable for Triangle {
    fn draw(&mut self, image: &mut Image) {
        // Drawing the three sides of the triangle
        Line {
            point_a: self.point_a,
            point_b: self.point_b,
        }
        .draw(image);
        Line {
            point_a: self.point_b,
            point_b: self.point_c,
        }
        .draw(image);
        Line {
            point_a: self.point_a,
            point_b: self.point_c,
        }
        .draw(image);
    }

    fn color(&mut self) {
        todo!()
    }
}

impl Drawable for Rectangle {
    fn draw(&mut self, image: &mut Image) {
        // Drawing the four sides of the rectangle
        Line {
            point_a: self.point_a,
            point_b: self.point_b,
        }
        .draw(image);
        Line {
            point_a: self.point_b,
            point_b: self.point_c,
        }
        .draw(image);
        Line {
            point_a: self.point_c,
            point_b: self.point_d,
        }
        .draw(image);
        Line {
            point_a: self.point_d,
            point_b: self.point_a,
        }
        .draw(image);
    }

    fn color(&mut self) {
        todo!()
    }
}

impl Drawable for Circle {
    fn draw(&mut self, image: &mut Image) {
        let x0 = self.center.x;
        let y0 = self.center.y;

        let mut x = self.radius;
        let mut y = 0;
        let mut p = 1 - self.radius; // Initial decision parameter

        // When radius is zero, only a single point will be printed at center
        if self.radius == 0 {
            Point { x: x0, y: y0 }.draw(image);
            return;
        }

        // Initial point on circle at the end of radius
        Point { x: x0 + x, y: y0 }.draw(image);
        Point { x: x0 - x, y: y0 }.draw(image);
        Point { x: x0, y: y0 + x }.draw(image);
        Point { x: x0, y: y0 - x }.draw(image);

        // Until the radius is greater than the y value
        while x > y {
            y += 1;

            if p <= 0 {
                p += 2 * y + 1;
            } else {
                x -= 1;
                p += 2 * y - 2 * x + 1;
            }

            // If the radius is greater than the y value
            if x < y {
                break;
            }

            Point {
                x: x0 + x,
                y: y0 - y,
            }
            .draw(image);
            Point {
                x: x0 - x,
                y: y0 - y,
            }
            .draw(image);
            Point {
                x: x0 + x,
                y: y0 + y,
            }
            .draw(image);
            Point {
                x: x0 - x,
                y: y0 + y,
            }
            .draw(image);

            if x != y {
                Point {
                    x: x0 + y,
                    y: y0 - x,
                }
                .draw(image);
                Point {
                    x: x0 - y,
                    y: y0 - x,
                }
                .draw(image);
                Point {
                    x: x0 + y,
                    y: y0 + x,
                }
                .draw(image);
                Point {
                    x: x0 - y,
                    y: y0 + x,
                }
                .draw(image);
            }
        }
    }

    fn color(&mut self) {
        todo!()
    }
}
