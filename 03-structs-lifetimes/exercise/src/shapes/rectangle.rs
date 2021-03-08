pub struct Rectangle {
    width: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let rectangle = Rectangle {
            width: 2.0f64,
            height: 3.0f64,
        };
        let actual = rectangle.area();
        let expected = 6.0;
        assert_eq!(actual, expected);
    }
}
