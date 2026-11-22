use rand::Rng;
use raster::Color;

use crate::geometrical_shapes::point::Point;
use crate::geometrical_shapes::{Displayable, Drawable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

#[allow(dead_code)]
impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: Point::new(center.x, center.y),
            radius: radius.max(0),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let width = width.max(1);
        let height = height.max(1);
        let center = Point::random(width, height);
        let max_radius = center
            .x
            .min(width - 1 - center.x)
            .min(center.y.min(height - 1 - center.y))
            .min(20);
        let radius = rand::thread_rng().gen_range(0..=max_radius);

        Self { center, radius }
    }
}

