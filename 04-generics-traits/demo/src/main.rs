trait Shape {
    fn area(&self) -> f64;
}

// You can derive traits (auto implementation)
#[derive(Clone, Copy, Debug, PartialEq)]
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// You can implement traits on other things
impl Shape for f64 {
    fn area(&self) -> f64 {
        *self
    }
}

// Advanced traits
// Trait objects (generics that implement a trait)
// By default, all traits have a size in memory known to Rust
// Trait objects cannot have this property hence the special marker ?Sized
struct JigsawPuzzle<T: ?Sized + Shape> {
    pieces: Vec<Box<T>>,
}

impl<T: ?Sized + Shape> JigsawPuzzle<T> {
    pub fn area(&self) -> f64 {
        let mut area = 0.0;

        for shape in self.pieces.iter() {
            area = area + shape.area();
        }

        area
    }
}

fn main() {
    let circle = Circle { radius: 1.0 };
    let rectangle = Rectangle {
        width: 1.0,
        height: 2.0,
    };

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle), Box::new(rectangle)];
    let jigsaw = JigsawPuzzle { pieces: shapes };
    let area = jigsaw.area();
    println!("The sum of the areas is {}", area);

    // More advanced traits
    //
    // Generics get more complicated when mixed with traits
    // It's because we don't know how large a Shape will take in memory
    // Rectangle has 2 fields; Circle 1
    // A Box is basically a safe pointer to some place in memory (the heap)
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle), Box::new(rectangle)];
    println!("The sum of the areas is {}", sum_area(shapes));
}

// Notice this ?Sized
// By default, Rust implements Sized on all generic functions
// Which is why `dyn Shape`
// I don't keep all of this in my head: the compiler points me to the right documentation
fn sum_area<T: ?Sized + Shape>(shapes: Vec<Box<T>>) -> f64 {
    let mut area = 0.0;

    for shape in shapes {
        area = area + shape.area();
    }

    area
}
