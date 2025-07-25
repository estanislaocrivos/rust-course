#![allow(dead_code)]
#![allow(unused_variables)]

use core::panic;
use rand::random;

mod project_id78;
use crate::project_id78::{alphabets, apply_to_jobs, is_even};

const THIS_IS_A_CONST: i32 = 5_000_000; // Equivalent to a macro in C (can be local or global, as in this case)

fn main() {
    /* ========================================================================================== */

    /* Data variables and mutability  */
    println!("Learning variables and mutability in Rust...");

    /* You can rename a type for convenience */
    type Meters = i32;
    let distance: Meters = 50; // This may be more convenient sometimes

    /* Immutable variables won't let you change their value once created and assigned one  */
    let random_number: u8 = random(); // This is an immutable variable
    println!("Hello, world! The random number is {}.", random_number);
    println!("I can also do this: {random_number}, to print the random_number variable value");
    println!("You can also re-print a value like this {0}, {0}, without having to add two random_number at the end", random_number);

    // random_number = 10; // This will throw an error

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

    /* Rust allows variable shadowing, which means that a variable can be re-created even after it was first created, even with different types */
    let x: i32 = 55;
    println!("x = {x}");
    let x: u8 = 5;
    println!("Shadowed x = {}", x);

    /* Large numbers can use _ as visual separators to clarify the multiple of 1000s positions */
    let x = 1_000_000; // One million

    /* The types usize and isize are defined based on the system architecture, if the processor for which the program will be compiled is 32-bit, then usize will be equivalent to a u32 type, the same for isize but signed */
    let x: usize = 45; // Eq. to u64 in this case

    /* ========================================================================================== */

    /* Data types  */
    println!("\nLearning data types in Rust...");

    /* Strings */

    // Static (immutable)
    let immut_string: &'static str = "Hello, world!"; // This is a static string, it is allocated on read-only memory (flash) and embedded on the binary file. Cannot be modified
    println!("{}", immut_string);

    // Dynamic (mutable)
    let mut mut_string: String = "This string could grow...".to_string(); // This is a heap-allocated string, so it could grow. New memory will be allocated on the heap for storing new characters.
    println!("{}", mut_string);
    mut_string += " Growing...";
    println!("{}", mut_string);

    /* Methods on data types */
    let x: i32 = -55;
    println!("abs(x) = {}", x.abs());
    println!("x^2 = {}", x.pow(2));
    let x = "  Hello, World!   ";
    println!("Trimmed string: {}", x.trim());
    let x: f64 = 3.44467;
    println!("ceil(x) = {}", x.ceil());
    println!("floor(x) = {}", x.floor());
    println!("Formatted float x = {:.2}", x); // Print with only 2 decimal places

    /* Casting. In C, for example, you would cast an uint16_t as an uint8_t by using (uint8_t)x. In Rust you use the as keyword */
    let x: f64 = 16.465;
    let y: u8 = x as u8; // Now y = 16
    println!("Casted x, y = {y}");

    /* Booleans */
    let x: bool = 2 == 2;
    println!("x = {}, not x = {}", x, !x);
    println!("x & !x = {}", x && !x); // Evals. to false

    /* Character type */
    let x: char = 'B';
    let y: char = '🔌';
    println!(
        "x = {}, is alphabetic? {}, is uppercase? {}, y = {}, is alphabetic? {}",
        x,
        x.is_alphabetic(),
        x.is_uppercase(),
        y,
        y.is_alphabetic()
    );

    /* Arrays can contain a single data type. Arrays have fixed size (cannot grow) */
    let array: [u8; 5] = [1, 2, 3, 4, 5]; // You can specify type and size, else it will be inferred
    println!("array[0] = {}", array[0]); // Access array elements with the [] notation
    let array = [6, 7, 8, 9, 10, 11, 12]; // It is not necessary to indicate the type or size
    const ARRAY_SIZE: usize = 10;
    let array: [u16; ARRAY_SIZE] = [0; ARRAY_SIZE]; // You may initialize all the elements to 0

    /* Debug trait. Every time we print something in the console using {} and then calling the variable to be printed, Rust first checks if the variable to be printed implements the display trait. A trait is an interface which must be implemented by the data type. In this case the Display trait allows a data type to be printed on the console. For example, arrays do not implement the Display trait, so we cannot to this: */
    // println!("The array is {}", array);
    /* For cases like this we can use the debug trait, which Rust offers to be able to print data types with little effort. We can invoke this trait by using :? inside the print macro:  */
    println!("The array is {:?}", array);

    /* dbg! macro. This macro allows to  print debug information by adding line and file to the output string. This avoid having to use print with formatting, etc. Should only use during development and debugging. */
    dbg!(2 + 2); // The data type must implement the debug trait.

    /* Tuples are data types which can contain multiple data types tied together */
    let tuple = (10, 20.5, 'H');
    let (_x, _y, _z) = tuple;
    println!("tuple[0] = {}", tuple.0); // Access tuple elements with the . operator (first index is 0)
    println!("tuple[1] = {}", tuple.1);
    println!("tuple[2] = {}", tuple.2);

    /* Range type. A range is generally used to iterate over an iterable */
    let range = 0..11; // Exclusive, goes up to the end - 1
    for element in range {
        println!("element = {}", element);
    }
    let letters = 'a'..'d';
    for letter in letters {
        println!("letter = {}", letter);
    }

    for element in array {
        println!("element = {}", element);
    }

    /* The unit type. This is what a function will return if there is nothing to be returned: */
    let unit_type = ();

    /* For example, this function will return a unit type: */
    fn mystery_function() {}
    println!("Call to mystery_function = {:?}", mystery_function());

    /* Blocks in functions. Blocks allows to achieve isolation, without needing to define a new function. All the variables which are created inside the block are not accesible after the block. */

    let calculation = {
        println!("I can access the array, defined before: {:?}", array);
        45 + 45
    };
    println!("calculation = {}", calculation);

    /* ========================================================================================== */

    /* Project ID78 solutions */
    println!("\nProject ID78 solutions:");
    apply_to_jobs(32, "Rust developer".to_string());
    println!(
        "Calling is_even with 9: {}, calling is_even with 8: {}",
        is_even(9),
        is_even(8)
    );
    println!(
        "Calling alphabets with aardvark: {:?}",
        alphabets("aardvark")
    );
    println!("Calling alphabets with zoology: {:?}", alphabets("zoology"));
    println!("Calling alphabets with zebra: {:?}", alphabets("zebra"));

    /* ========================================================================================== */

    /* Control flows */
    println!("\nLearning control flows in Rust...");

    /* if-else structures will be ignored (already used) */

    /* Match structures are kind of switch-case in C. Matches let you match the result of an expression, and act upon */
    let evaluation: bool = true;
    match evaluation {
        true => println!("evaluation is true!"),
        false => println!("evaluation is false!"),
    }

    /* You can also assign the result of the match statement */
    let result = match evaluation {
        true => println!("evaluation is true!"),
        false => println!("evaluation is false!"),
    };

    /* And you can match multiple values: */
    let number = 5;
    match number {
        4 | 5 | 6 => println!("Number is 4, 5 or 6!"),
        _ => println!("Number has any other value!"), // You use _ for default cases or any uncovered cases
    }

    /* Loops */

    let mut seconds = 60;
    loop {
        seconds -= 1;
        if seconds == 0 {
            break;
        }
        continue; // Continue forces to go back to loop()
                  // The program never reaches here...
    }

    /* This loop has the same behaviour as the previous one */
    seconds = 60;
    while seconds > 0 {
        seconds -= 1;
    }

    /* ========================================================================================== */

    /* Functions */
    println!("\nLearning functions in Rust...");

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

    /* ========================================================================================== */

    /* Structs */
    println!("\nLearning structs in Rust...");

    struct User {
        name: String,
        surname: String,
        age: u8,
        email: String,
        active: bool,
    }

    let mut user = User {
        name: "John".to_string(),
        surname: "Appleseed".to_string(),
        age: 50,
        email: "johnappleseed@apple.com".to_string(),
        active: true,
    };

    println!(
        "name = {}, surname = {}, age = {}, email = {}, active = {}",
        user.name, user.surname, user.age, user.email, user.active
    ); // Struct fields are accessed with the . operator

    user.age += 1; // I can do this because the user structure is mutable

    fn new_user(name: String, surname: String, age: u8, email: String) -> User {
        return User {
            name,
            surname,
            age,
            email,
            active: true,
        }; // Rust can automatically detect the name-match between the struct fields and the function parameters and assign them directly. Else, you can do typical assignment.
    }

    let new_user = new_user(
        "John".to_string(),
        "Doe".to_string(),
        55,
        "johndoe@gmail.com".to_string(),
    );

    println!(
        "new_user info: name = {}, surname = {}",
        new_user.name, new_user.surname
    );

    let another_user = User {
        name: "Anna".to_string(),
        ..user // This tells Rust to fill the rest of unassigned parameters with the contents of the user struct.
    };

    println!(
        "another_user info: name = {}, age = {}",
        another_user.name, another_user.age
    );

    /* You can implement methods for a structure, like classes in OOP */

    impl User {
        fn read_age(&self) {
            println!("The user age is: {}", self.age);
        }
    }

    new_user.read_age(); // Access methods with the . operator

    /* ========================================================================================== */

    /* Enums */
    println!("\nLearning enums in Rust...");

    enum UserRole {
        BASIC,
        ADMIN,
    }

    enum Website {
        LINKEDIN(String),
        PERSONAL(String),
    }

    struct EnhancedUser {
        name: String,
        surname: String,
        age: u8,
        email: String,
        active: bool,
        role: UserRole,
        web: Website,
    }

    let user = EnhancedUser {
        name: "John".to_string(),
        surname: "Appleseed".to_string(),
        age: 88,
        email: "johnappleseed@apple.com".to_string(),
        active: true,
        role: UserRole::ADMIN,
        web: Website::LINKEDIN("johnappleseed.linkedin.com".to_string()),
    };

    fn has_access(role: UserRole) -> bool {
        match role {
            UserRole::ADMIN => return true,
            UserRole::BASIC => return false,
        } // This match structure can be compared to a switch-case structure in C
    }

    if has_access(user.role) {
        println!("The user has access!");
    } else {
        println!("The user does not have access!");
    } // Introducing if-else blocks

    /* ========================================================================================== */

    /* Option enum */

    let mut name: Option<String> = None; // Here I'm telling Rust that name is a string but for now it does not hold anything. If I try to print name Rust will not let me. I need to write a string to name and then use a match to operate with it (like a NULL-checking in C)

    match name {
        Some(name) => println!("Name is {}", name),
        None => println!("Name is None!"),
    } // Here you can check if name is None or not and then if not, you can use it normally.

    name = Some("John".to_string()); // You use Some() to assign a value to a Option<> variable. Else you can assign None.

    match name {
        Some(name) => println!("Name is {}", name),
        None => println!("Name is None!"),
    }

    /* You use Option<type> (you can read it as optional type) to indicate that a variable may or may not have a value, so it may be null. A typical use of this type is when the user asked to input a string, for example, which may be empty. It can optionally be a string or be None. */

    fn get_name(name_or_none: bool) -> Option<String> {
        if name_or_none {
            return Some("John".to_string());
        } else {
            return None;
        }
    }

    let name = get_name(true);
    match name {
        Some(name) => println!("Getting name: {}", name),
        None => (),
    }

    /* Another way of checking for a value in an Optional. This is a more "verbose" or understandable way to check for a value */
    let name: Option<String> = Some("John".to_string());
    if let Some(name_str) = name {
        println!("The name is: {}", name_str);
    }

    /* ========================================================================================== */

    /* Generics */
    println!("\nLearning generics in Rust...");

    struct Point<T> {
        x: T,
        y: T,
    } // Using <T> allows to use different types on x and y, may be both floats, both ints or both strings, for example.

    let _point = Point { x: 1.0, y: 2.5 };

    /* ========================================================================================== */

    /* Traits */
    println!("\nLearning traits in Rust...");

    struct Human;
    struct Cat;

    /* A trait can be used to define an interface */
    trait Talk {
        fn say_hello(&self) -> String;
    }

    /* Then you can implement the interface for each new "class" */

    impl Talk for Human {
        fn say_hello(&self) -> String {
            return "Hola!".to_string();
        }
    }

    impl Talk for Cat {
        fn say_hello(&self) -> String {
            return "Miau!".to_string();
        }
    }

    let john = Human;
    println!("John says {}", john.say_hello());

    let chichi = Cat;
    println!("Chichi says {}", chichi.say_hello());

    /* ========================================================================================== */

    /* Iterators */
    println!("\nLearning iterators in Rust...");

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

    /* ========================================================================================== */

    /* Closures */
    println!("\nLearning closures in Rust...");

    /* First, in Rust we can assign a function to a variable */
    let sum_fn = sum;

    println!("The sum of 2 + 5 is {}", sum_fn(2, 5));

    /* Closures are what lambdas in other languages. These are functions which are anon. (do not have a certain name), can only contain a single expression, and are usually defined inline (single line expression)   */

    let sum = |num1: i32, num2: i32| -> i32 { num1 + num2 };

    println!("The sum of 8 + 1 is {}", sum(8, 1));

    /* ========================================================================================== */

    /* Let-else block */
    println!("\nLearning let-else in Rust...");

    let mut some_var: Option<u8> = Some(10);

    /* To "decode" an optional, you may use the already-known match{} structure: */
    match some_var {
        Some(number) => println!("some_var = {}", number),
        None => println!("some_var = None"),
    }

    /* Using match{} forces the user to include all the logic for handling the number inside the match structure. If you want to use the number outside the match scope you cannot. Another way of "matching" which allows the user to use the variable later in the code is:  */

    let Some(number) = some_var else {
        /* Here you MUST diverge (panic or return) */
        panic!("Panicking!");
    };

    /* This allows you to use the number here... */

    println!("The number is {}", number);

    /* ========================================================================================== */

    /* Labeled blocks */
    println!("\nLearning labeled blocks in Rust...");

    some_var = Some(0);
    'first_block: {
        let Some(mut number) = some_var else {
            break 'first_block;
        };
        for index in 0..10 {
            number += 1;
            println!("number = {}", number);
            if index > 5 {
                println!("Exiting 'first_block");
                break 'first_block;
            }
        }
    }

    /* ========================================================================================== */

    /* Collections */
    println!("\nLearning collections in Rust...");

    /* Collections are types of data which allow to change size during program execution */

    /* Vectors. All its elements must be of the same type. */

    // let mut v: Vec<u8> = Vec::new();
    // v = vec![1, 2, 3]; // Use the vec! macro for initializing the vector.
    // v.push(4);
    // for element in v {
    //     println!("element = {}", element);
    // }
}
