use crate::geometrical_shapes::point::Point;

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

