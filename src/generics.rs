// src/generics.rs

pub fn generics_in_rust() {
    /* Generics allow us to define things such as functions or structs with generic types of data, making the struct or the function generic in terms of the data types it accepts/stores/returns */

    // T is the name I choose to give to the parameter, could be anything else. The angled brackets tell Rust that this function will accept/return a generic
    fn generic_function<T>(variable: T) -> T {
        return variable;
    }

    /* On both examples below, Rust will compile this code by duplicating the function, one for each type of data, although we defined the function only once */

    let variable = generic_function(10); // The parameter of the function is an i32 (automatically assumed by Rust), then the function will return it (by its definition), making variable an i32

    let string = generic_function("Hello, world!".to_string()); // In this case the function returns a String, because I passed a String

    // You can also mix defined types with generic types
    fn return_tuple<T>(a: T, b: i32) -> (T, i32) {
        return (a, b);
    }

    // Multiple generics
    fn multiple_generics<T, U>(a: T, b: U) -> (T, U) {
        return (a, b);
    }

    struct Point<T> {
        x: T,
        y: T,
        z: i32,
    } // Using <T> allows to use different types on x and y, may be both floats, both ints or both strings, for example.

    let point = Point {
        x: 1.0,
        y: 2.5,
        z: 32,
    };

    // You can implement methods on structs with generic fields but for certain generic type, for example the method below is only useful on objects where all three coordinates are i32
    impl Point<i32> {
        fn print_point(&self) {
            println!("(x, y, z) = ({}, {}, {})", self.x, self.y, self.z);
        }
    }

    let point = Point { x: 1, y: 2, z: 3 };
    point.print_point();

    // Here I cannot call the print_point method because I am using floats as T (there is no implementation of the method for float)
    let point = Point {
        x: 1.0,
        y: 2.0,
        z: 3,
    };

    // If I want to implement generic methods for Point (which can be used by any implementation of Point) I have to do this:
    impl<T> Point<T> {
        fn set_x_coordinate(&mut self, x: T) {
            self.x = x;
        }
    }
    let mut point = Point { x: 1, y: 2, z: 3 };
    point.set_x_coordinate(5);
    println!("(x, y, z) = ({}, {}, {})", point.x, point.y, point.z);

    // You may also use generics as associated data on enums
    enum FoodTopping<T> {
        Basic,
        Special(T),
    }
    let topping = FoodTopping::Special("Tomato");
}
