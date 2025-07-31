// src/options.rs

pub fn options_in_rust() {
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

    /* Result enum */

    /* The result enum can be used to express success or error, as these are its two possible values. Rust does not have exceptions but the result enum helps to serve the same purpose. When a function is set to return a Result enum, the compiler forces to handle both cases (ok and err). Result is of type Result<T,E>, where T is the parameter it returns in the Ok case and E is the parameter it returns in the Err case, which can be of the same or of different type. For example, on success you may return an integer and on error you may return a string. */

    let ok: Result<i32, &str> = Result::Ok(10);
    let error: Result<i32, &str> = Result::Err("Error");

    /* Let-else block */

    let some_var: Option<u8> = Some(10);

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
}
