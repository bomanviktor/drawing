use rand::Rng;
pub mod geometrical_shapes {
    use raster::{Color, Image};

    pub trait Drawable {
        fn draw(&mut self, image: &mut Image);
        fn color() -> Color;
    }

    pub trait Displayable {
        fn display(&mut self, x: i32, y: i32, color: Color);
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point {
            x, y
        }
    }

    pub fn random(max_width: i32, max_height: i32) -> Point {
        Point {
            x: rand_num(max_width),
            y: rand_num(max_height)
        }
    }
}

pub struct Line {
    point_a: Point,
    point_b: Point
}

impl Line {
// Need to figure out implementation of the referenced "Point"
    pub fn new(point_a: &Point, point_b: &Point) -> Line {
        Line {
            point_a: Point::new(point_a.x, point_a.y),
            point_b: Point::new(point_b.x, point_b.y)
        }
    }

    pub fn random(max_width: i32, max_height: i32) -> Line {
        Line {
            point_a: Point::random(max_width, max_height),
            point_b: Point::random(max_width, max_height)
        }
    }
}

pub struct Triangle {
    point_a: Point,
    point_b: Point,
    point_c: Point
}

impl Triangle {
    pub fn new(point_a: &Point, point_b: &Point, point_c: &Point) -> Triangle {
        Triangle {
            point_a: Point::new(point_a.x, point_a.y),
            point_b: Point::new(point_b.x, point_b.y),
            point_c: Point::new(point_c.x, point_c.y)
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
    point_a: Point,
    point_b: Point,
    point_c: Point,
    point_d: Point
}

impl Rectangle {
    pub fn new(point_a: &Point, point_c: &Point) -> Rectangle {
        Rectangle {
            point_a: Point::new(point_a.x, point_a.y),
            point_b: Point::new(point_c.x, point_a.y),
            point_c: Point::new(point_c.x, point_c.y),
            point_d: Point::new(point_a.x, point_c.y),

        }
    }
}

pub struct Circle {
    center: Point,
    radius: i32
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Circle {
        Circle {
            center: Point::new(center.x, center.y),
            radius
        }
    }

    pub fn random(max_width: i32, max_height: i32) -> Circle {
        Circle {
            center: Point::random(max_width, max_height),
            radius: rand_num(max_height)
        }
    }
}

// generate a tuple with two random numbers in the range of image_width and image_height
pub(crate) fn rand_num(max_range: i32) -> i32 {
    rand::thread_rng().gen_range(..=max_range)
}

