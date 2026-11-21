use rand::Rng;
use raster::Color;

use crate::geometrical_shapes::{Displayable, Drawable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width.max(1)),
            y: rng.gen_range(0..height.max(1)),
        }
    }
}

impl Drawable for Point {
    fn color(&self) -> Color {
        Color::rgb(0, 255, 0) // green
    }

    fn draw(&self, image: &mut dyn Displayable) {
        image.display(self.x, self.y, self.color());
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn random_points_stay_inside_bounds() {
        for _ in 0..100 {
            let point = Point::random(100, 80);

            assert!((0..100).contains(&point.x));
            assert!((0..80).contains(&point.y));
        }
    }
}
