// src/strings.rs

pub fn strings_in_rust() {
    // Static (immutable)
    let immut_string: &'static str = "Hello, world!"; // This is a static string, it is allocated on read-only memory (flash) and embedded on the binary file. Cannot be modified
    println!("{}", immut_string);

    // Dynamic (mutable)
    let mut mut_string: String = "This string could grow...".to_string(); // This is a heap-allocated string, so it could grow. New memory will be allocated on the heap for storing new characters. In this case the variable mut_string is stored in the stack, this variable holds a reference to a point in the heap where the string is allocated and its size
    println!("{}", mut_string);
    mut_string.push_str(" Growing...");
    println!("Mutated string: {}", mut_string);

    let string = "Hello, world!";
    let string: String = "Goodbye, world!".to_string();
}
