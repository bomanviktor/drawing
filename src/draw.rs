use crate::geometrical_shapes::*;
use std::f64::consts::PI;

use raster::{Color, Image};

impl Drawable for Point {
    fn draw(&mut self, image: &mut Image) {
        if self.x >= 0 && self.x < image.width && self.y >= 0 && self.y < image.height {
            image
                .set_pixel(self.x, self.y, self.color.to_owned())
                .unwrap();
        }
    }

    fn color(&mut self, color: &Color) -> &mut Self {
        self.color = color.clone();
        self
    }
}

impl Drawable for Line {
    fn draw(&mut self, image: &mut Image) {
        // Implementing Bresenham's Line Algorithm
        let mut center_x = self.point_a.x;
        let mut center_y = self.point_a.y;
        let x1 = self.point_b.x;
        let y1 = self.point_b.y;

        let dx = (x1 - center_x).abs();
        let dy = (y1 - center_y).abs();

        let sx = if center_x < x1 { 1 } else { -1 };
        let sy = if center_y < y1 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            if center_x >= 0 && center_x < image.width && center_y >= 0 && center_y < image.height {
                image
                    .set_pixel(
                        (center_x as usize).try_into().unwrap(),
                        (center_y as usize).try_into().unwrap(),
                        self.color.to_owned(),
                    )
                    .unwrap();
            }

            if center_x == x1 && center_y == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                center_x += sx;
            }
            if e2 < dx {
                err += dx;
                center_y += sy;
            }
        }
    }

    fn color(&mut self, color: &Color) -> &mut Self {
        self.color = color.clone();
        self
    }
}

impl Drawable for Triangle {
    fn draw(&mut self, image: &mut Image) {
        let color = &self.color;

        Line::new(&self.point_a, &self.point_b)
            .color(color)
            .draw(image);
        Line::new(&self.point_b, &self.point_c)
            .color(color)
            .draw(image);
        Line::new(&self.point_a, &self.point_c)
            .color(color)
            .draw(image);
    }

    fn color(&mut self, color: &Color) -> &mut Self {
        self.color = color.clone();
        self
    }
}

impl Drawable for Rectangle {
    fn draw(&mut self, image: &mut Image) {
        let color = &self.color;

        Line::new(&self.point_a, &self.point_b)
            .color(color)
            .draw(image);
        Line::new(&self.point_b, &self.point_c)
            .color(color)
            .draw(image);
        Line::new(&self.point_c, &self.point_d)
            .color(color)
            .draw(image);
        Line::new(&self.point_d, &self.point_a)
            .color(color)
            .draw(image);
    }

    fn color(&mut self, color: &Color) -> &mut Self {
        self.color = color.clone();
        self
    }
}

impl Drawable for Circle {
    fn draw(&mut self, image: &mut Image) {
        let center_x = self.center.x;
        let center_y = self.center.y;
        let color = &self.color;

        let mut radius = self.radius;
        let mut y = 0;
        let mut p = 1 - self.radius; // Initial decision parameter

        if self.radius == 0 {
            return;
        }

        // Initial point on circle at the end of radius
        Point::new(center_x - radius, center_y)
            .color(color)
            .draw(image);
        Point::new(center_x, center_y + radius)
            .color(color)
            .draw(image);
        Point::new(center_x, center_y - radius)
            .color(color)
            .draw(image);

        // Until the radius is greater than the y value
        while radius > y {
            y += 1;

            if p <= 0 {
                p += 2 * y + 1;
            } else {
                radius -= 1;
                p += 2 * y - 2 * radius + 1;
            }

            // If the radius is greater than the y value
            if radius < y {
                break;
            }

            Point::new(center_x + radius, center_y - y)
                .color(color)
                .draw(image);
            Point::new(center_x - radius, center_y - y)
                .color(color)
                .draw(image);
            Point::new(center_x + radius, center_y + y)
                .color(color)
                .draw(image);
            Point::new(center_x - radius, center_y + y)
                .color(color)
                .draw(image);

            if radius != y {
                Point::new(center_x + y, center_y - radius)
                    .color(color)
                    .draw(image);
                Point::new(center_x - y, center_y - radius)
                    .color(color)
                    .draw(image);
                Point::new(center_x + y, center_y + radius)
                    .color(color)
                    .draw(image);
                Point::new(center_x - y, center_y + radius)
                    .color(color)
                    .draw(image);
            }
        }
    }

    fn color(&mut self, color: &Color) -> &mut Self {
        self.color = color.clone();
        self
    }
}

impl Drawable for Pentagon {
    fn draw(&mut self, image: &mut Image) {
        let color = &self.color;
        for angle in 0..5 {
            let rad1 = (angle * 72 + self.rotation - 90) as f64 * PI / 180.0;
            let rad2 = ((angle + 1) * 72 + self.rotation - 90) as f64 * PI / 180.0;
            let x1 = self.center.x + (self.radius as f64 * rad1.cos()) as i32;
            let y1 = self.center.y + (self.radius as f64 * rad1.sin()) as i32;
            let x2 = self.center.x + (self.radius as f64 * rad2.cos()) as i32;
            let y2 = self.center.y + (self.radius as f64 * rad2.sin()) as i32;
            Line::new(&Point::new(x1, y1), &Point::new(x2, y2))
                .color(color)
                .draw(image);
        }
    }

    fn color(&mut self, color: &Color) -> &mut Self {
        self.color = color.clone();
        self
    }
}

impl Drawable for Cube {
    fn draw(&mut self, image: &mut Image) {
        let color = &self.color;

        let w = self.width;
        let a1 = &self.top_left;
        let a3 = Point::new(a1.x + w, a1.y + w);
        let b1 = Point::new(a1.x + w / 2, a1.y + w / 2);
        let b3 = Point::new(a1.x + w + w / 2, a1.y + w + w / 2);

        Rectangle::new(a1, &a3).color(color).draw(image);
        Rectangle::new(&b1, &b3).color(color).draw(image);

        Line::new(&a1, &b1).color(color).draw(image);
        Line::new(&a3, &b3).color(color).draw(image);

        Line::new(
            &Point::new(a1.x + w, a1.y),
            &Point::new(a1.x + w + w / 2, a1.y + w / 2),
        )
        .color(color)
        .draw(image);

        Line::new(
            &Point::new(a1.x, a1.y + w),
            &Point::new(a1.x + w / 2, a1.y + w + w / 2),
        )
        .color(color)
        .draw(image);
    }
    fn color(&mut self, color: &Color) -> &mut Self {
        self.color = color.clone();
        self
    }
}
