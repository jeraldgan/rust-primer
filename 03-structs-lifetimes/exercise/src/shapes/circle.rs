pub struct Circle {}

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
