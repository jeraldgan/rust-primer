mod other_source_file;

fn main() {
    // Variables
    let x = 1;
    println!("{}", x);
    dbg!(x);
    // x = 2;

    // Mutability
    let mut y = 2;
    dbg!(y);
    y = 3;
    dbg!(y);

    // Casting
    // let z: f64 = 1;
    // let z: f64 = 1 as f64;
    // let z: f64 = (f64) 1;
    // let z: f64 = 1.0f64;
    // let z: u8 = 2; // !!!

    // Iteration
    // let numbers = [1, 2, 3, 4, 5];
    // for n in numbers {
    //     dbg!(n);
    //     n = n + 2;
    // }
    // for n in numbers.iter() {
    //     *n = n + 2;
    // }

    // Destructuring
    let x = (1, 2);
    let (first, second) = x;
    dbg!(first, second);

    // Iterations + mutability
    let mut numbers = [1, 2, 3, 4, 5];
    for n in numbers.iter_mut() {
        *n = *n + 2;
    }

    // Ranges, conditionals
    for i in 0..10 {
        if i % 5 == 0 {
            println!("fizzbuzz");
        } else {
            println!("{}", i);
        }
    }

    // Iterators
    let numbers = [1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
    dbg!(doubled);

    // Functions
    fn double(x: &i32) -> i32 {
        // return x * 2;

        // No semicolon = implicit return
        x * 2
    }
    dbg!(double(&2));
    let doubled: Vec<i32> = numbers.iter().map(double).collect();
    dbg!(doubled);

    // Importing functions
    let tripled: Vec<i32> = numbers.iter().map(other_source_file::triple).collect();
    dbg!(tripled);

    use other_source_file::triple;
    let tripled: Vec<i32> = numbers.iter().map(triple).collect();
    dbg!(tripled);

    // Strings? Later.
}
