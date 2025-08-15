/* This project corresponds to task number 210 @ course https://www.udemy.com/course/learn-to-code-with-rust/ */

/*
Define a Food struct with a single `name` field
set to a String. Derive a Debug implementation.

Define a Restaurant struct with a `reservations` field
set to a u32 and a `has_mice_infestation` field set to
a bool. Derive a Debug implementation.

Define a `chef_special` method on the Restaurant.
The method will return the restaurant's famous
dish. It should return an Option containing a Food
struct.

If the restaurant has a mice infestation, return the
None variant. There is no chef special!

If the restaurant has less than 12 reservations, return
a Food instance with a name of "Uni Sashimi" wrapped in
the Some variant. If it has 12 or more reservations,
return a Food instance with a name of "Strip Steak"
instead, also wrapped in the Some variant.

Define a `deliver_burger` method on the Restaurant.
It should accept an `address` string slice; it will
represent where to deliver the order. It should
return a Result type where the Ok variant holds a Food
struct and the Err variant holds a String.

If the restaurant has a mice infestation, return the
Err variant containing a String of "Sorry, we have a
mice problem".

If the user's address is an empty string, return the Err
variant with a String of "No delivery address specified".
HINT: You can use the `is_empty` method on a string to check
if it has 0 characters.
https://doc.rust-lang.org/std/string/struct.String.html#method.is_empty

Otherwise, the delivery is good to go! Return the Ok
variant containing a Food struct with a `name` of "Burger".

In the `main` function, create a `Restaurant` instance
with 11 reservations and a mice infestation.

Invoke the `chef_special` method and print out its return
value. It should be the None variant.

Invoke the `deliver_burger` method with an argument of "123
Elm Street" and print out its return value. It should be
the Err variant.

Create another `Restaurant` instance with 15 reservations
and no mice infestation.

Invoke the `chef_special` method and print out its return
 value. It should be the Some variant with a "Strip Steak".

Invoke the `deliver_burger` method with an argument of an
empty address. Print out its return value. It should be the
Err variant.

Invoke the `deliver_burger` method again with an argument
of a valid address. Print out its return value. It should
be the Ok variant nesting a Food struct with a `name` of
"Burger".
*/

#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if self.has_mice_infestation {
            return None;
        }
        if self.reservations < 12 {
            return Some(Food {
                name: "Uni Sashimi".to_string(),
            });
        } else {
            return Some(Food {
                name: "Strip Steak".to_string(),
            });
        }
    }

    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err("Sorry, we have a mice infestation...".to_string());
        }
        if address.is_empty() {
            return Err("No delivery address was specified...".to_string());
        }
        return Ok(Food {
            name: "Burger".to_string(),
        });
    }
}

pub fn project_id210_solution() {
    fn solution_variants(reservations: u32, has_mice_infestation: bool, address: &str) {
        let restaurant = Restaurant {
            reservations,
            has_mice_infestation,
        };
        let option = restaurant.chef_special();
        match option {
            Some(food) => println!("The chef special is {}", food.name),
            None => println!("There is no chef special!"),
        }
        let option = restaurant.deliver_burger(address);
        match option {
            Ok(food) => println!("The delivery will send a {}", food.name),
            Err(string) => println!("{}", string),
        }
    }
    solution_variants(11, true, "123 Elm Street");
    solution_variants(11, false, "123 Elm Street");
    solution_variants(12, true, "");
    solution_variants(12, false, "");
    solution_variants(12, true, "123 Elm Street");
    solution_variants(12, false, "123 Elm Street");
}
