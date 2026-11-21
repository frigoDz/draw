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

