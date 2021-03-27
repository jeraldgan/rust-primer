mod shapes;

use shapes::{Circle, Rectangle};

fn main() {
    let rect = Rectangle {
        width: 2.0f64,
        height: 3.0f64,
    };
    println!("Area of a 2 x 3 rectangle is {:?}", rect.area());

    let circ = Circle { radius: 1.0f64 };
    println!("Area of a unit circle is {:?}", circ.area());
}

mod tests {
    use super::*;

    #[test]
    fn mutatability() {
        fn mutate_circle_double_radius(circle: &mut Circle) -> () {
            circle.radius = circle.radius * 2.0;
        }

        // Mutable structs
        let mutable_circle = Circle { radius: 1.0 };
        mutate_circle_double_radius(mutable_circle);
        println!("{}", mutable_circle.area());

        // We can't do this without specifying equality for Circle
        // assert_eq!(mutable_circle, Circle { radius: 2.0 });
        assert_eq!(mutable_circle.area(), Circle { radius: 2.0 }.area());
    }

    #[test]
    fn lifetimes_borrow_checker() {
        // This is an implicit lifetime example
        // When you get programming more complex stuff you will encounter explicit lifetimes
        fn take_ownership_of_my_vec<T>(_: Vec<T>) {}

        // Who owns my_vec now?
        let my_vec = vec![0, 1, 2, 3];

        // Think of where my_vec is going and who's holding ownership
        take_ownership_of_my_vec(my_vec);
        dbg!(my_vec.get(3));
    }
}
