pub mod circle;
pub mod line;
pub mod point;
pub mod rectangle;
pub mod triangle;

pub use circle::Circle;
pub use line::Line;
pub use point::Point;
pub use rectangle::Rectangle;
pub use triangle::Triangle;

use raster::Color;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn color(&self) -> Color;
    fn draw(&self, image: &mut dyn Displayable);
}

#[cfg(test)]
mod tests;
