// src/control_flows.rs

pub fn control_flows_in_rust() {
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
}
