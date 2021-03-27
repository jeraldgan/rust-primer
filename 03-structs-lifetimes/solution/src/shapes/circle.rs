// If you have forgotten; don't worry I've got you:
// The area of a circle is πr^2, or Pi * radius * radius
// https://doc.rust-lang.org/std/primitive.f64.html
// https://doc.rust-lang.org/std/f64/consts/index.html
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let circle = Circle { radius: 1.0f64 };
        let actual = circle.area();
        let expected = std::f64::consts::PI;
        assert!((actual - expected).abs() < std::f64::EPSILON);
    }
}
