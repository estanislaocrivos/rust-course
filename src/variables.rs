// src/variables.rs

const THIS_IS_A_CONST: i32 = 5_000_000; // Equivalent to a macro in C (can be local or global, as in this case)

pub fn variables_in_rust() {
    /* You can rename a type for convenience */
    type Meters = i32;
    let distance: Meters = 50;

    /* Immutable variables won't let you change their value once created and assigned one  */
    let random_number: u8 = 8; // This is an immutable variable
    println!("Hello, world! The random number is {}.", random_number);
    println!("I can also do this: {random_number}, to print the random_number variable value");
    println!("You can also re-print a value like this {0}, {0}, without having to add two 'random_number' at the end", random_number);

    // random_number = 10; // This will throw an error because random_number is immutable

    /* Mutable variables values can be changed any time */
    let mut mut_var: u8 = 255;
    println!("mut_var = {} at creation", mut_var);
    mut_var = 0;
    println!("mut_var = {} after modification", mut_var);

    /* Rust allows the definition of consts, which are like macros in C. By def. their value cannot be changed and they could be defined inside a scope or globally */
    println!("THIS_IS_A_CONST = {}", THIS_IS_A_CONST);

    /* By default rust assumes that all integers are i32, and all floats are f64, unless we specify each type (preferrable on embedded programming due to the different chip architectures and variables size) */
    let uint8_t_var: u8 = 255;
    println!(
        "Unsigned integers of 8 bits can only hold up to (2^8 - 1) = {}",
        uint8_t_var
    );

    /* Rust allows variable shadowing, which means that a variable can be redefined even after it was first created, even with different types */
    let x: i32 = 55;
    println!("x = {x}");
    let x: u8 = 5;
    println!("Shadowed x = {}", x);

    /* Large numbers can use _ as visual separators to clarify the multiple of 1000s positions */
    let x = 1_000_000; // One million

    /* The types usize and isize are defined based on the system architecture, if the processor for which the program will be compiled is 32-bit, then usize will be equivalent to a u32 type, the same for isize but signed */
    let x: usize = 45; // Eq. to u64 in this case (usize depends on the architecture)
}
