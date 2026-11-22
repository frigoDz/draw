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

impl Drawable for Circle {
    fn color(&self) -> Color {
        Color::rgb(255, 165, 0)
    }

    fn draw(&self, target: &mut dyn Displayable) {
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
                    target.display(point_x, point_y, self.color());
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
    use super::{Circle, Point};

    #[test]
    fn random_circle_stays_inside_positive_bounds() {
        for _ in 0..100 {
            let circle = Circle::random(100, 80);

            assert!(circle.center.x - circle.radius >= 0);
            assert!(circle.center.y - circle.radius >= 0);
            assert!(circle.center.x + circle.radius < 100);
            assert!(circle.center.y + circle.radius < 80);
        }
    }

    #[test]
    fn random_circle_fits_single_pixel_image() {
        let circle = Circle::random(1, 1);

        assert_eq!(circle.center, Point::new(0, 0));
        assert_eq!(circle.radius, 0);
    }
}
