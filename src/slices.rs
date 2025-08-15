// src/slices.rs

pub fn slices_in_rust() {
    /* Slices are portions of data structures such as strings or arrays */

    let string = "John Appleseed".to_string();
    let name = &string[..4]; // Here the name variable is now a reference to the first 4 bytes (from 0 to 3) of the string. Bytes, not characters (in some cases 1 byte != 1 char), although in this case it does match
    let surname = &string[4..];
    println!("Name: {name}");
    println!("Surname: {surname}");

    /* Using the type &str as parameters in functions allows to pass not only fixed strings but also slices and REFS. to heap-allocated strings. This is more versatile: */
    fn print_something(string: &str) {
        println!("Something: {string}");
    }
    print_something(&"Hello! As a heap-allocated string".to_string()); // Here I pass a reference to a heap-allocated string, this is valid
    print_something("Hello! As a static string"); // I can also pass a static string
    let string = "Hello, cut this!";
    print_something(&string[..5]); // And also a slice (ref. to a string)

    /* You may also "slice" an array */

    let mut array = [0, 1, 2, 3, 4, 5];
    let array_slice = &array[..3];

    fn print_array(array_slice: &[i32]) {
        // You may not define the size of the slice, this allows for more generic function parameters
        println!("Array: {:?}", array_slice);
    }
    print_array(&array);
    print_array(&array_slice);

    /* Mutability on slices: Rust does not allow mutable string slices, but it does allow mutable array slices */

    println!("Here, the value of array[0] is {}", array[0]);
    let mut_array_slice: &mut [i32] = &mut array[..3];
    mut_array_slice[0] = 20; // Modify array using the mutable reference to the slice
    println!("Here, the value of array[0] is {}", array[0]);
}
