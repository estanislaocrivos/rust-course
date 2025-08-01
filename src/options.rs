// src/options.rs

pub fn options_in_rust() {
    /* Options or optionals in Rust let you define a variable wich may or may not hold a value. These variables can hold a value of the type specified, or they may hold None. The user must unwrap the Optional using a certain control structure, such as a match{} */

    let mut name: Option<String> = None; // Here I'm telling Rust that name may hold a String, but for now it holds None

    println!("name: {:?}", name); // Optionals do not implement the display trait, although they implement the debug trait

    // To unwrap the optional I need to use some kind of match structure, like match{}:
    match name {
        Some(name) => println!("Name is {}", name), // Here I can use the variable normally
        None => println!("Name is None!"),
    }

    name = Some("John".to_string()); // You use Some() to assign a value to an optional

    match name {
        Some(name) => println!("Name is {}", name),
        None => println!("Name is None!"),
    }

    /* A typical use of this type is when the user asked to input a string, for example, which may be empty. It can optionally be a string or be None. */
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
        None => println!("No name!"),
    }

    /* The if-let structure offers another way of checking for a value in an Optional. This control flow allows the execution of a dedicated block in case the optional holds a value or holds None */
    let name: Option<String> = Some("John".to_string());
    if let Some(name_str) = name {
        println!("The name is: {}", name_str);
    }

    /* Result enum */

    /* The result enum is implemented by Rust, and can be used to express success (Ok) or error (Err). When a function is set to return a Result enum, the compiler forces to handle both cases. Result is of type Result<T,E>, where T is the parameter/s it returns in the Ok case and E is the parameter/s it returns in the Err case, which can be of the same or of different type. For example, on success you may return an integer and on error you may return a string. */

    let ok: Result<i32, &str> = Result::Ok(10);
    let ok: Result<(u8, u8), &str> = Result::Ok((1, 2)); // You may return a tuple, or eventually a struct, on any of the two options
    let error: Result<i32, &str> = Result::Err("Error");

    /* Let-else blocks allow to handle one of the cases on an optional and forces to panic or end execution given the condition. For example, we can say "if this optional is None, then panic, else continue execution" */

    let optional: Option<u8> = Some(10);

    let Some(number) = optional else {
        /* Here you MUST diverge (panic or return) */
        panic!("Panicking!");
    };
    println!("The number is {}", number); // If not none, you can use number here
}
