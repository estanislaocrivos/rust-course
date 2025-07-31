// src/ownership.rs

pub fn ownership_in_rust() {
    let time = 2025;
    let year = time; // This generates a copy of the variable time, so now there are two vars. on the stack (time, year) with the value 2025. This is because these types of vars. implement the copy trait (integers, floats, bools, etc.)
    println!("The time is {time}, and the year is {year}");

    let string = "Hello".to_string();
    let another_string = string; // Strings do not implement the copy trait, so doing this will not generate a copy of the string. There will be still only one string in the heap, and now the 'string' variable looses ownership of the original string. MOVES OWNERSHIP.

    drop(another_string); // This drops the memory allocated by the variable, making the variable no longer usable

    let mut string: String = "Goodbye, world!".to_string();
    let another_string = string.clone(); // Now there are two strings on the heap, each one with its owner
    println!("The clone method allows to generate a copy of the heap-allocated string: {string}, {another_string}");

    /* References */

    let mut value = 5;
    println!("value = {value}");
    let value_reference = &mut value;
    *value_reference += 1;
    println!("value + 1 (modified by reference) = {value}");

    let string_reference = &mut string;
    string_reference.push_str(" Adding string to string...");
    println!("String 'string' after modification by reference: {string}");

    let static_string = "Hello"; // This variable is of type &str because static_string is a reference to the point in static memory where the string is located
    let static_string_ref = static_string; // This variable also refers to the same string which is stored in flash (embedded in the binary file)

    let number = 4;
    fn print_number(number: i32) {
        println!("Number is {number}...");
    }
    print_number(number); // A copy of the variable number is passed to the function, because integers do implement the copy trait
    println!("I can still use the variable number here: {number}");

    let string: String = "Hello!".to_string();
    fn print_string(string: String) {
        println!("By passing a String to a function, the function takes ownership of it: {string}");
    }
    print_string(string);
    println!("Here, string does not exist anymore :(");

    let string: String = "Hello!".to_string();
    fn print_string_and_return(string: String) -> String {
        println!("By passing a String to a function, the function takes ownership of it: {string}, but this time this function will return it...");
        return string;
    }
    let string = print_string_and_return(string);
    println!("Here, the string exists because the function returned the ownership: {string}");

    let string: String = "Hello!".to_string();
    fn print_string_from_ref(string_ref: &String) {
        println!("Here, the function takes a reference to the string, so it can still live after this call: {string_ref}");
    }
    print_string_from_ref(&string);
    println!("I can still use the string after the call: {string}");

    let mut string: String = "Burger and ".to_string();
    println!("Original string: {string}");
    fn add_food(original: &mut String, added: &str) {
        original.push_str(added);
    }
    add_food(&mut string, "fries...");
    println!("Modified string: {string}");
}
