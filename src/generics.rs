// src/generics.rs

pub fn generics_in_rust() {
    struct Point<T> {
        x: T,
        y: T,
    } // Using <T> allows to use different types on x and y, may be both floats, both ints or both strings, for example.

    let point = Point { x: 1.0, y: 2.5 };
}
