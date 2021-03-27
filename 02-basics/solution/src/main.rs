fn main() {}

fn mutability() -> u8 {
    let mut x = 1;
    x = 2;
    return x;
}

#[test]
fn test_mutability() {
    assert_eq!(mutability(), 2);
}

fn casting() -> f64 {
    let x: u8 = 1;
    x as f64
}

#[test]
fn test_casting() {
    assert_eq!(casting(), 1.0);
}

fn collections() -> u8 {
    let xs = [1, 2, 3, 4, 5];

    // Return the sum
    let mut sum = 0;
    for n in xs.iter() {
        sum = sum + n;
    }
    return sum;

    // Uusing iterators:
    // xs.iter().fold(0, |acc, x| acc + x)
}

#[test]
fn test_collections() {
    assert_eq!(collections(), 15);
}
