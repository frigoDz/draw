# drawing

A small Rust program that draws a point, a line, a rectangle, a triangle and a
bunch of circles on an image. Every shape has its own color. The result is
saved as `image.png`.

## How to run

    cargo run      # creates image.png
    cargo test     # runs the unit tests

## How it works

- `main.rs` creates the image and draws all the shapes.
- `geometrical_shapes` holds the shapes and two traits: `Drawable` (to draw a
  shape) and `Displayable` (to put a pixel on the image).
- The line is drawn with Bresenham's algorithm. The rectangle and the triangle
  reuse it for their edges.

## Authors

- hzmarrou: traits, Point, Line, main.rs
- aelyoussef: Rectangle, Triangle, Circle, tests
