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
        let center = Point::random(width, height);
        let radius = rand::thread_rng().gen_range(1..=width.min(height).max(2) / 2);

        Self { center, radius }
    }
}

impl Drawable for Circle {
    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color::rgb(rng.gen_range(80..=255), rng.gen_range(80..=255), rng.gen_range(80..=255))
    }

    fn draw(&self, target: &mut dyn Displayable) {
        let color = self.color();
        let center_x = i64::from(self.center.x);
        let center_y = i64::from(self.center.y);
        let mut x = i64::from(self.radius);
        let mut y = 0;
        let mut decision = 1 - x;

        while x >= y {
            let points = [
                (center_x + x, center_y + y),
                (center_x + y, center_y + x),
                (center_x - y, center_y + x),
                (center_x - x, center_y + y),
                (center_x - x, center_y - y),
                (center_x - y, center_y - x),
                (center_x + y, center_y - x),
                (center_x + x, center_y - y),
            ];

            for (point_x, point_y) in points {
                if let (Ok(point_x), Ok(point_y)) = (i32::try_from(point_x), i32::try_from(point_y))
                {
                    target.display(point_x, point_y, color.clone());
                }
            }

            y += 1;
            if decision <= 0 {
                decision += 2 * y + 1;
            } else {
                x -= 1;
                decision += 2 * (y - x) + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;

    #[test]
    fn random_circle_center_stays_inside_bounds() {
        for _ in 0..100 {
            let circle = Circle::random(100, 80);

            assert!((0..100).contains(&circle.center.x));
            assert!((0..80).contains(&circle.center.y));
            assert!(circle.radius >= 1);
        }
    }
}
