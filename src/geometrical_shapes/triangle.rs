use raster::Color;

use crate::geometrical_shapes::line::draw_segment;
use crate::geometrical_shapes::point::Point;
use crate::geometrical_shapes::{Displayable, Drawable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self {
            a: Point::new(a.x, a.y),
            b: Point::new(b.x, b.y),
            c: Point::new(c.x, c.y),
        }
    }
}

impl Drawable for Triangle {
    fn color(&self) -> Color {
        Color::rgb(255, 0, 0)
    }

    fn draw(&self, target: &mut dyn Displayable) {
        let color = self.color();
        draw_segment(target, &self.a, &self.b, color.clone());
        draw_segment(target, &self.b, &self.c, color.clone());
        draw_segment(target, &self.c, &self.a, color);
    }
}

#[cfg(test)]
mod tests {
    use super::{Point, Triangle};

    #[test]
    fn constructor_copies_each_vertex() {
        let a = Point::new(1, 2);
        let b = Point::new(3, 4);
        let c = Point::new(5, 6);
        let triangle = Triangle::new(&a, &b, &c);

        assert_eq!(triangle.a, a);
        assert_eq!(triangle.b, b);
        assert_eq!(triangle.c, c);
    }
}
