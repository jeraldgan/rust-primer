mod circle;
mod rectangle;

pub use circle::Circle;
pub use rectangle::Rectangle;

pub trait Shape {
  fn area(&self) -> f64;
}
