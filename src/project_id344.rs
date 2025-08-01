/* This project corresponds to task number 344 @ course https://www.udemy.com/course/learn-to-code-with-rust/ */

/*
Define a 'double_the_length' function that accepts a
reference to a vector and returns a usize. The function
should be able to accept a reference to a vector storing
any data type.

Return the length of the vector (you can acquire this
with the 'len' method) times 2. The function accepts a
reference parameter.

Examples:
double_the_length(&vec![1, 2, 3]))    => 6
double_the_length(&vec![1, 2, 3, 4])) => 8

Does this function need lifetime annotations?
Explain why or why not.

---

Define a `last_two` function that accepts a slice as
a parameter. This function will eventually be invoked
with a &Vec<T>, but we want to use deref coercion to
accept it as a slice. Return a slice as well.

The function should return a slice with the last two
elements of the input.

Examples:
last_two(&vec![1, 2, 3])           => [2, 3]
last_two(&vec![1, 2, 3, 4, 5, 6])  => [5, 6]

Does this function need lifetime annotations?
Explain why or why not.

---

Define a 'first_five' function that accepts two string
slice parameters: 'text' and 'announcement'. The function
should print the value of 'announcement' and return a
slice of the first 5 bytes of 'text'.

Example:
first_five("refrigerator", "Hello") => "refri"

Does this function need lifetime annotations?
Explain why or why not.

--

Define a `find_string_that_has_content` function that
accepts three string slice parameters: `first`,
`second`, and `target`.

Either the first or second string will hold the `target`
substring. Return the string that contains the `target`
content. You can use the `contains` method on a string
to check if holds a substring.

Example:
find_string_that_has_content("programming", "dining", "gram")
=> "programming"

Does this function need lifetime annotations?
Explain why or why not.
*/

// No lifetime annotations needed because it is only returning an integer and using an immutable reference to the vector to calculate it, does not own the vector
fn double_the_length<T>(vector: &Vec<T>) -> usize {
    return vector.len() * 2;
}

// No lifetime annotations needed because Rust infers that the return reference has to do with the parameter reference
fn last_two<T>(slice: &[T]) -> &[T] {
    return &slice[slice.len() - 2..];
}

// Lifetime specifier needed because you need to tell Rust that you will return a reference to a flash-stored string so it needs to be static, will exist before, during, and after the function call, on flash memory
fn first_five(text: &'static str, announcement: &str) -> &'static str {
    println!("{}", announcement);
    return &text[..5];
}

// Same reason as the previous one
fn find_string_that_has_content(
    first: &'static str,
    second: &'static str,
    target: &str,
) -> &'static str {
    if first.contains(target) {
        return first;
    }
    if second.contains(target) {
        return second;
    }
    return "";
}

pub fn project_id344_solution() {
    println!("{}", double_the_length(&vec![1, 2, 3]));
    println!("{:?}", last_two(&vec![1, 2, 3]));
    println!(
        "{}",
        find_string_that_has_content("programming", "dining", "gram")
    );
}
