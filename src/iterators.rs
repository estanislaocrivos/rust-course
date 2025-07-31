// src/iterators.rs

pub fn iterators_in_rust() {
    let some_array: [u8; 6] = [1, 2, 3, 4, 5, 6];

    let mut idx = 0;
    for elmnt in some_array {
        println!("The element number {} is: {}", idx, elmnt);
        idx += 1;
    }

    for elmnt in some_array.iter() {
        println!("The element is: {}", elmnt);
    }

    /* Iterator is a trait. Every array or vector automatically contain the implementations of the methods inside a trait. Given that Iterator is a trait, you can create your own implementation */

    struct Counter {
        index: u8,
        array: [u8; 16],
    }

    impl Iterator for Counter {
        type Item = u8;

        /* In this implementation, next will give the next element only if the index is < 5 */
        fn next(&mut self) -> Option<Self::Item> {
            self.index += 1;
            if self.index < 5 {
                return Some(self.index);
            } else {
                return None;
            }
        }
    }

    let mut c = Counter {
        index: 0,
        array: [0; 16],
    };

    while c.index < 10 {
        match c.next() {
            Some(value) => println!("index = {}", value),
            None => println!("None!"),
        }
    }
}
