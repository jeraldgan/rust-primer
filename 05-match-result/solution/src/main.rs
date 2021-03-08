fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum Sculpture {
    Bunny,
    Teapot,
}

fn parse_value(input: &str) -> Option<Sculpture> {
    // return Sculpture::Teapot;
    match input {
        "bunny" => Some(Sculpture::Bunny),
        "teapot" => Some(Sculpture::Teapot),
        _ => None,
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_optional_enum() {
        assert_eq!(parse_value("bunny"), Some(Sculpture::Bunny));
        assert_eq!(parse_value("teapot"), Some(Sculpture::Teapot));
        assert_eq!(parse_value("taco"), None);
    }

    #[test]
    fn test_using_optional() {
        let wrapper = Some("value");
        let value = wrapper.expect("some useful message");
        assert_eq!(value, "value");
    }
}
