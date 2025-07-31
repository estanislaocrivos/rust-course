// src/data_types.rs

pub fn data_types_in_rust() {
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
}
