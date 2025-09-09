// src/closures.rs

pub fn closures_in_rust() {
    /* Closures are what lambdas in other languages. These are functions which are anon. (do not have a name tag), can only contain a single expression, and are usually defined inline (single line expression)   */

    /* Closures can access outside variables, which classical functions cannot. */

    /* This is a classical  */
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    /* First, in Rust we can assign a function to a variable */
    let sum_fn = sum;

    println!("The sum of 2 + 5 is {}", sum_fn(2, 5));

    let sum = |num1: i32, num2: i32| -> i32 { num1 + num2 };

    println!("The sum of 8 + 1 is {}", sum(8, 1));
}
