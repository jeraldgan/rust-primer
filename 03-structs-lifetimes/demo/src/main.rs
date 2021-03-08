// Data
#[derive(Clone, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// What you do with it
// This is Nice. We can do more things with `impl` later.
impl Rectangle {
    // If you're familiar with Python syntax, we need explicit self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // No formal constructor, you can use whatever function name you want
    // Note the use of `Self`, this can be `Rectangle`
    fn new(width: u32, height: u32) -> Self {
        Self {
            width: width,
            height: height,
        }

        // Or: use syntatic sugar
        // Self { width, height }
    }
}

fn main() {
    let rect = Rectangle {
        width: 5,
        height: 10,
    };

    // Create a new clone to move it into the double_area_rect
    // In Rust, data must be copied or borrowed (referenced) to access it across scopes
    double_area_rect(rect.clone());
    double_area_rect(rect);

    // Or make the function take a reference instead (which is more performant)
    // dbg!(double_area_rect_reference(&rect));

    // Know that this is not a complete understanding of lifetimes, there's more to it
    //
    // Not covered: How to move trait objects, move across functions/threads
    //
    // But the principle is that data
    // Why lifetimes?
    //
    // Two benefits
    // 1. Allows for memory management
    //   - Rust has no runtime
    //   - = no built-in GC
    //   - memory management needs it to clean up references when safe
    //      - compiler does it at compile time
    //      - we don't have to manually implement destructors and keep track of lifetimes ourselves
    //
    // 2. Prevents use after free bugs
    //   Consider this C pseudocode
    //   ```c
    //   char* ptr = (char*)malloc (SIZE);
    //   free(ptr);
    //   printf("%s", *ptr);
    //   ```
    //
    // Rust prevents this
}

fn double_area_rect(rect: Rectangle) -> u32 {
    rect.area() * 2
}

fn double_area_rect_reference(rect: &Rectangle) -> u32 {
    rect.area() * 2
}
