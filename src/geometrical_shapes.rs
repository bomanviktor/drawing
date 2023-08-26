use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&mut self, image: &mut Image);
    fn color(&mut self, color: &Color) -> &mut Self;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: Color
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y, color: Color::white() }
    }

    pub fn random(max_width: i32, max_height: i32) -> Point {
        Point {
            x: rand_i32(max_width),
            y: rand_i32(max_height),
            color: rand_color()
        }
    }
}

pub struct Line {
    pub point_a: Point,
    pub point_b: Point,
    pub color: Color
}

impl Line {
    // Need to figure out implementation of the referenced "Point"
    pub fn new(point_a: &Point, point_b: &Point) -> Line {
        Line {
            point_a: point_a.clone(),
            point_b: point_b.clone(),
            color: Color::white()
        }
    }

    pub fn random(max_width: i32, max_height: i32) -> Line {
        Line {
            point_a: Point::random(max_width, max_height),
            point_b: Point::random(max_width, max_height),
            color: rand_color()
        }
    }
}

pub struct Triangle {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,
    pub color: Color
}

impl Triangle {
    pub fn new(point_a: &Point, point_b: &Point, point_c: &Point) -> Triangle {
        Triangle {
            point_a: point_a.clone(),
            point_b: point_b.clone(),
            point_c: point_c.clone(),
            color: Color::white()
        }
    }
}

/*      EDITORS NOTE:
 *
 *      The points given are always A and C, and are always opposite of each other, example below:
 *
 *      This point -> A------B
 *                    |      |
 *                    D------C <- And this point
 *
 *      Another example:
 *
 *                        B------A <- This point
 *                        |      |
 *      And this point -> C------D
 *
 *      B is calculated by taking the y-position of A, and the X position of C.
 *      D is calculated by taking the x-position of A, and the Y position of C.
 *
 */
pub struct Rectangle {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,
    pub point_d: Point,
    pub color: Color
}

impl Rectangle {
    pub fn new(point_a: &Point, point_c: &Point) -> Rectangle {
        Rectangle {
            point_a: Point::new(point_a.x, point_a.y),
            point_b: Point::new(point_c.x, point_a.y),
            point_c: Point::new(point_c.x, point_c.y),
            point_d: Point::new(point_a.x, point_c.y),
            color:   Color::white()
        }
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub color: Color
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Circle {
        Circle {
            center: center.clone(),
            radius,
            color: Color::white()
        }
    }

    pub fn random(max_width: i32, max_height: i32) -> Circle {
        let max_range: i32;
        if max_width >= max_height {
            max_range = max_width;
        } else {
            max_range = max_height;
        }

        Circle {
            center: Point::random(max_width, max_height),
            radius: rand_i32(max_range),
            color: rand_color()
        }
    }
}


pub struct Pentagon {
    pub point_a: Point,
    pub point_b: Point,
    pub point_c: Point,
    pub point_d: Point,
    pub point_e: Point,
    pub color: Color
}

impl Pentagon {
    pub fn new(point_a: &Point, point_b: &Point, point_c: &Point, point_d: &Point, point_e: &Point) -> Pentagon {
        Pentagon {
            point_a: point_a.clone(),
            point_b: point_b.clone(),
            point_c: point_c.clone(),
            point_d: point_d.clone(),
            point_e: point_e.clone(),
            color: Color::white()
        }
    }
}

pub struct Cube {
    pub front: Rectangle,
    pub rear: Rectangle,
    pub top_left: Line,
    pub top_right: Line,
    pub bottom_right: Line,
    pub bottom_left: Line,
    pub color: Color
}

impl Cube {
    pub fn new(point_a: &Point, point_b: &Point, point_c: &Point) -> Cube {
        let front = Rectangle::new(point_a, point_b);
        let rear = Rectangle::new(point_c, &Point::new((
            point_b.x - point_c.x) + point_b.x, (point_b.y - point_a.y) + point_c.y));
        Cube {
            top_left: Line::new(&front.point_a, &rear.point_a),
            top_right: Line::new(&front.point_b, &rear.point_b),
            bottom_right: Line::new(&front.point_c, &rear.point_c),
            bottom_left: Line::new(&front.point_d, &rear.point_d),
            front,
            rear,
            color: Color::white()
        }
    }
}

// generate a random int in the range of image_width and image_height
fn rand_i32(max_range: i32) -> i32 {
    rand::thread_rng().gen_range(0..=max_range)
}

// generate a random u8
fn rand_u8() -> u8 {
    rand::thread_rng().gen_range(0..=std::u8::MAX)
}

fn rand_color() -> Color {
    Color {
        r: rand_u8(),
        g: rand_u8(),
        b: rand_u8(),
        a: 255
    }
}

pub fn back_cube_color(color: &Color) -> Color {
    Color {
        r: color.r.clone(),
        g: color.g.clone(),
        b: color.b.clone(),
        a: rand::thread_rng().gen_range(50..=150),
    }
}
