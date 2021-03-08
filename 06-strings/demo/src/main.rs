use std::str::FromStr;

fn main() {
    let s: &str = "Hello, world!";

    // Convert into String
    let s2: String = String::from_str(s).unwrap(); // s could be invalid bytes for a String
    let s2: String = s.to_owned();
    // String implements FromStr trait so Rust can automatically convert it with .into()
    let s2: String = s.into();

    // Convert back to str
    // s3 lives as long as the String s2 exists
    let s3: &str = s2.as_str();
}
