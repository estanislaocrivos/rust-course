// src/project_id78.rs

/* This project corresponds to task number 78 @ course https://www.udemy.com/course/learn-to-code-with-rust/ */

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
