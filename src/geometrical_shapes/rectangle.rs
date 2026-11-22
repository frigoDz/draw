use raster::Color;

use crate::geometrical_shapes::line::draw_segment;
use crate::geometrical_shapes::point::Point;
use crate::geometrical_shapes::{Displayable, Drawable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let left = p1.x.min(p2.x);
        let right = p1.x.max(p2.x);
        let top = p1.y.min(p2.y);
        let bottom = p1.y.max(p2.y);

        Self {
            p1: Point::new(left, top),
            p2: Point::new(right, bottom),
        }
    }
}

impl Drawable for Rectangle {
    fn color(&self) -> Color {
        Color::rgb(0, 150, 255)
    }

    fn draw(&self, target: &mut dyn Displayable) {
        let color = self.color();
        let top_left = Point::new(self.p1.x, self.p1.y);
        let top_right = Point::new(self.p2.x, self.p1.y);
        let bottom_right = Point::new(self.p2.x, self.p2.y);
        let bottom_left = Point::new(self.p1.x, self.p2.y);

        draw_segment(target, &top_left, &top_right, color.clone());
        draw_segment(target, &top_right, &bottom_right, color.clone());
        draw_segment(target, &bottom_right, &bottom_left, color.clone());
        draw_segment(target, &bottom_left, &top_left, color);
    }
}

#[cfg(test)]
mod tests {
    use super::{Point, Rectangle};
    use crate::geometrical_shapes::{Displayable, Drawable};
    use raster::Color;

    #[derive(Default)]
    struct RecordingDisplay(Vec<(i32, i32)>);

    impl Displayable for RecordingDisplay {
        fn display(&mut self, x: i32, y: i32, _color: Color) {
            self.0.push((x, y));
        }
    }

    #[test]
    fn constructor_normalizes_reversed_corners() {
        let rectangle = Rectangle::new(&Point::new(150, 300), &Point::new(50, 60));

        assert_eq!(rectangle.p1, Point::new(50, 60));
        assert_eq!(rectangle.p2, Point::new(150, 300));
    }

    #[test]
    fn draw_traces_all_four_edges() {
        let rectangle = Rectangle::new(&Point::new(1, 1), &Point::new(4, 3));
        let mut target = RecordingDisplay::default();

        rectangle.draw(&mut target);

        for corner in [(1, 1), (4, 1), (4, 3), (1, 3)] {
            assert!(target.0.contains(&corner));
        }
    }
}
