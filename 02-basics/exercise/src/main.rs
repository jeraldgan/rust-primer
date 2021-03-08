fn main() {}

fn mutability() -> u8 {
    let x = 1;
    return x;
}

#[test]
fn test_mutability() {
    assert_eq!(mutability(), 2);
}



fn casting() -> f64 {
    let x: u8 = 1;
    0.0
}

#[test]
fn test_casting() {
    assert_eq!(casting(), 1.0);
}



fn collections() -> u8 {
    let xs = [1, 2, 3, 4, 5];

    // Return the sum
    return 0;
}

#[test]
fn test_collections() {
    assert_eq!(collections(), 15);
}
