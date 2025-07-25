// src/project_id78.rs

/* This project corresponds to task number 78 @ course https://www.udemy.com/course/learn-to-code-with-rust/ */

/*
Define an apply_to_jobs function that accepts a
'number' parameter (an i32) and a 'title' parameter
(a string). It should print out the string:
"I'm applying to {number} {title} jobs".

Example:
apply_to_jobs(35, "Rust Developer")
-> "I'm applying to 35 Rust Developer jobs"

Define an is_even function that accepts a 'number'
parameter (an i32). The function should return a true
if the number is even and a false if the number is
odd.
Examples:
is_even(8) -> true
is_even(9) -> false

Define an alphabets function that accepts a 'text'
parameter (an &str). The function should return a
tuple of two Booleans. The first Boolean should check
if the text contains the letter 'a'. The second
Boolean should check if the text contains the letter
'z'. You can use the 'contains' method to check if a
string contains a specific character. See the documentation:
https://doc.rust-lang.org/std/primitive.str.html#method.contains

Examples:
println!("{:?}", alphabets("aardvark")); -> (true, false)
println!("{:?}", alphabets("zoology"));  -> (false, true)
println!("{:?}", alphabets("zebra"));    -> (true, true)
*/

pub fn apply_to_jobs(number: i32, title: String) {
    println!("I am applying to {} {} jobs!", number, title);
}

pub fn is_even(number: i32) -> bool {
    number % 2 == 0
}

pub fn alphabets(string: &str) -> (bool, bool) {
    let first_bool: bool = string.contains('a');
    let second_bool: bool = string.contains('z');
    (first_bool, second_bool)
}
