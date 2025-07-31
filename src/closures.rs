// src/closures.rs

pub fn closures_in_rust() {
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    /* First, in Rust we can assign a function to a variable */
    let sum_fn = sum;

    println!("The sum of 2 + 5 is {}", sum_fn(2, 5));

    /* Closures are what lambdas in other languages. These are functions which are anon. (do not have a certain name), can only contain a single expression, and are usually defined inline (single line expression)   */

    let sum = |num1: i32, num2: i32| -> i32 { num1 + num2 };

    println!("The sum of 8 + 1 is {}", sum(8, 1));
}
