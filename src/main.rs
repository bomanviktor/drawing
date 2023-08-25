mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new (
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl gs::geometrical_shapes::Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}

impl gs::geometrical_shapes::Drawable for gs::Point {
    fn color() -> Color {
        Color{
            r: gs::rand_num(255) as u8,
            g: gs::rand_num(255) as u8,
            b: gs::rand_num(255) as u8,
            a: gs::rand_num(255) as u8
        }
    }
    fn draw(&self, image: &mut Image){
        let color = gs::geometrical_shapes::Drawable::color();
        image.set_pixel(self.x, self.y, color).expect("TODO: panic message");
    }

}