use std::str::FromStr;

fn main() {
    let s: &str = "Hello, world!";

    // Convert into String
    let s2: String = String::from_str(s).unwrap(); // s could be invalid bytes for a String
    let s2: String = s.to_owned();
    // String implements the FromStr trait 
    // Rust converts the &str slice into String using `into`
    // `into` is automatically implemented if FromStr is implemented
    // https://doc.rust-lang.org/std/convert/trait.Into.html
    let s2: String = s.into();

    // Convert back to str
    // s3 lives as long as the String s2 exists
    let s3: &str = s2.as_str();
}
