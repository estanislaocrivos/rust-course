// src/functions.rs

pub fn functions_in_rust() {
    fn hello_world_fn() {
        println!("This function simply prints something...");
    }

    hello_world_fn();

    fn sum(a: u8, b: u8) -> u16 {
        a as u16 + b as u16
    }

    let a: u8 = 0xFF;
    let b: u8 = 0xFF;

    println!("Sum of {} and {} is {}", a, b, sum(a, b));

    fn take_ownership_of_string(text: String) {
        println!("The string is '{}'. This function takes ownership of the string and does not return it.", text);
    }

    let text: String =
        "This string will not be available after passing it to the function".to_string();
    take_ownership_of_string(text);

    // text += "Cannot do this." // This cannot be done because the function never returned ownership of the mutable string

    /* But what you can do is pass a reference to a string, this string could be modified inside the function (if the ref. is mutable) or not and then the string will still be available on the main flow. */

    fn not_take_ownership_of_string(text: &String) {
        println!("From not_take_ownership_of_string: The string is {}", text);
    }

    let mut text: String =
        "This string will be available after passing it to the function.".to_string();

    not_take_ownership_of_string(&text);

    text += " Can be modified after function call.";
    println!("The text can be still used after function call: {}", text);
}
