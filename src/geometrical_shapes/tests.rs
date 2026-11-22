use super::{Circle, Displayable, Drawable, Point, Rectangle, Triangle};
use raster::Color;

#[derive(Default)]
struct RecordingDisplay(Vec<(i32, i32)>);

impl Displayable for RecordingDisplay {
    fn display(&mut self, x: i32, y: i32, _color: Color) {
        self.0.push((x, y));
    }
}

#[test]
fn rectangle_is_unchanged_when_corner_order_is_reversed() {
    let first = Rectangle::new(&Point::new(150, 300), &Point::new(50, 60));
    let reversed = Rectangle::new(&Point::new(50, 60), &Point::new(150, 300));

    assert_eq!(first, reversed);
}

#[test]
fn triangle_draws_each_of_its_three_edges() {
    let triangle = Triangle::new(&Point::new(1, 1), &Point::new(4, 1), &Point::new(2, 4));
    let mut target = RecordingDisplay::default();

    triangle.draw(&mut target);

    for vertex in [(1, 1), (4, 1), (2, 4)] {
        assert!(target.0.contains(&vertex));
    }
}

#[test]
fn circle_draws_points_on_each_side_of_its_center() {
    let circle = Circle::new(&Point::new(5, 5), 2);
    let mut target = RecordingDisplay::default();

    circle.draw(&mut target);

    for point in [(7, 5), (3, 5), (5, 7), (5, 3)] {
        assert!(target.0.contains(&point));
    }
}
