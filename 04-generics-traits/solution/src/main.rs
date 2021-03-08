fn main() {}

mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_derive() {
        #[derive(Debug)]
        struct ShouldBeDebuggable {
            foo: String,
        }

        let debuggable = ShouldBeDebuggable { foo: "bar".into() };
        dbg!(debuggable);
    }

    #[test]
    fn test_generic_struct() {
        struct Point<T> {
            _x: i32,
            value: T,
        };

        // You need to supply the correct generics here
        let mut map: HashMap<i32, Point<f64>> = HashMap::new();

        // Then insert Points into `map` to make the assertions true
        map.insert(-1, Point { _x: 1, value: 1.0 });
        map.insert(2, Point { _x: 2, value: 2.0 });

        // Note `unwrap`: we will be going into this next chapter
        assert_eq!(map.get(&-1).unwrap().value, 1.0);
        assert_eq!(map.get(&2).unwrap().value, 2.0);
    }

    #[test]
    fn test_generic_function() {
        trait HasWeight {
            fn get_weight(&self) -> i32;
        };
        // The weight of a cow is defined as weight(meat + milk)
        struct Cow {
            meat_weight: i32,
            milk_weight: i32,
        }

        // The weight of a cat is the number of furballs it has
        struct Cat {
            furballs: i32,
        }

        // Implement the trait for the structs and then
        impl HasWeight for Cow {
            fn get_weight(&self) -> i32 {
                self.meat_weight + self.milk_weight
            }
        }

        impl HasWeight for Cat {
            fn get_weight(&self) -> i32 {
                self.furballs
            }
        }

        // Then write the get_weight function that works for either Cat or Cow
        fn get_weight<T: HasWeight>(animal: T) -> i32 {
            animal.get_weight()
        }

        assert_eq!(
            get_weight(Cow {
                meat_weight: 1,
                milk_weight: 5
            }),
            6
        );
        assert_eq!(get_weight(Cat { furballs: 3 }), 3);
    }
}
