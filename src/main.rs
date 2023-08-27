mod draw;
mod geometrical_shapes;

use crate::geometrical_shapes::Drawable;
use geometrical_shapes as gs;
use gs::Displayable;

use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let mut rectangle = gs::Rectangle::new(&gs::Point::new(150, 450), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);

    let mut pentagon = gs::Pentagon::new(&gs::Point::new(500, 300), 200, 0, &Color::white());

    pentagon.draw(&mut image);

    let mut cube = gs::Cube::new(&gs::Point::new(500, 500), 200, &Color::white());

    cube.draw(&mut image);

    let mut triangle = gs::Triangle::new(
        &gs::Point::new(200, 500),
        &gs::Point::new(150, 700),
        &gs::Point::new(400, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
