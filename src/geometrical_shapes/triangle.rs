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

