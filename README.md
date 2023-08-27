# Drawing

Drawing is a Rust-based project that allows users to draw various shapes on an image. It provides a simple and efficient way to add shapes like circles, rectangles, and polygons to your images.

## Description

Drawing is designed to be user-friendly and efficient. It provides a variety of shapes that can be drawn on an image. The shapes available include but are not limited to circles, rectangles, and polygons. The project is implemented in Rust, taking advantage of its performance and safety features.

## Installation

To install and run the Drawing project, you need to have Rust installed on your machine. If you don't have Rust installed, you can download it from the official [Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone the Drawing project from its repository and run it using the following commands:

```bash
git clone https://01.gritlab.ax/git/takeme/drawing.git
cd drawing
cargo run
```

## Usage

To use the Drawing project, you can create a new Rust file and import the Drawing library. Here's a basic example of how to draw a circle:

```rust
    // Import the Drawing library
    use drawing::shapes::Circle;
    // Define image size and initialize image
    let mut image = Image::new(800, 800);
    // Create the shape you are going to draw
    let circle = Circle::new(Point::new(400, 400), 50);
    // Draw it on the image
    image.draw(&circle);
```

## Other Examples

Here are some examples of how to use the Drawing project:

Drawing a rectangle:

```rust
    use drawing::shapes::Rectangle;

    let mut image = Image::new(800, 800);
    let rectangle = Rectangle::new(Point::new(200, 200), 100, 200);
    image.draw(&rectangle);
```

Drawing a cube:

```rust
    use drawing::shapes::Cube;

    let mut image = Image::new(800, 800);
    let cube = Cube::new(Point::new(300, 300), 100);
    image.draw(&cube);
```

Drawing a polygon:

```rust
    use drawing::shapes::Polygon;
    
    let polygon = Polygon::new(vec![Point::new(100, 100), Point::new(200, 100), Point::new(150, 200)]);
    image.draw(&polygon);
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or create an issue if you find a bug or have a feature request.

## License

This project is licensed under the MIT License.

## Contact

If you have any questions or feedback, please feel free to contact:

- [Sagar Yadav](https://github.com/sagarishere)
- [Viktor Boman](https://github.com/bomanviktor)
- [Bao Ta](https://github.com/tathienbao)
