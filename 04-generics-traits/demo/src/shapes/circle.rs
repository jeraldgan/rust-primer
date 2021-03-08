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
