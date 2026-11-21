use raster::Color;

use crate::geometrical_shapes::point::Point;
use crate::geometrical_shapes::{Displayable, Drawable};

/// Draws a straight line from `a` to `b` (Bresenham's algorithm).
/// Rectangle and Triangle reuse this function for their edges.
pub fn draw_segment(image: &mut dyn Displayable, a: &Point, b: &Point, color: Color) {
    let (mut x, mut y) = (a.x as i64, a.y as i64);
    let (end_x, end_y) = (b.x as i64, b.y as i64);

    let dx = (end_x - x).abs();
    let dy = -(end_y - y).abs();
    let step_x = if x < end_x { 1 } else { -1 };
    let step_y = if y < end_y { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        image.display(x as i32, y as i32, color.clone());
        if x == end_x && y == end_y {
            break;
        }

        let double_error = 2 * error;
        if double_error >= dy {
            error += dy;
            x += step_x;
        }
        if double_error <= dx {
            error += dx;
            y += step_y;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

#[allow(dead_code)]
impl Line {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self {
            a: Point::random(width, height),
            b: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn color(&self) -> Color {
        Color::rgb(255, 0, 255) // magenta
    }

    fn draw(&self, image: &mut dyn Displayable) {
        draw_segment(image, &self.a, &self.b, self.color());
    }
}

#[cfg(test)]
mod tests {
    use super::{draw_segment, Line, Point};
    use crate::geometrical_shapes::Displayable;
    use raster::Color;

    #[derive(Default)]
    struct Recorder(Vec<(i32, i32)>);

    impl Displayable for Recorder {
        fn display(&mut self, x: i32, y: i32, _color: Color) {
            self.0.push((x, y));
        }
    }

    #[test]
    fn random_line_endpoints_stay_inside_bounds() {
        for _ in 0..100 {
            let line = Line::random(100, 80);

            for p in [&line.a, &line.b] {
                assert!((0..100).contains(&p.x));
                assert!((0..80).contains(&p.y));
            }
        }
    }

    #[test]
    fn segment_includes_both_endpoints() {
        let mut rec = Recorder::default();
        draw_segment(
            &mut rec,
            &Point::new(1, 2),
            &Point::new(4, 2),
            Color::rgb(0, 0, 0),
        );

        assert_eq!(rec.0, [(1, 2), (2, 2), (3, 2), (4, 2)]);
    }

}
