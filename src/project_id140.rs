/* This project corresponds to task number 140 @ course https://www.udemy.com/course/learn-to-code-with-rust/ */

/*
Define a `cereals` array with 5 heap Strings
  - Cookie Crisp
  - Cinnamon Toast Crunch
  - Frosted Flakes
  - Cocoa Puffs
  - Captain Crunch

Declare a `first_two` variable that extracts a slice
of the first two cereals. Print the slice.

Declare a `mid_three` variable that extracts a slice
of the middle three cereals (Cinnamon Toast Crunch
up to and including Cocoa Puffs). Print the slice.

Declare a `last_three` variable that extracts a slice
of the last three cereals. Print the slice.

Using the `last_three` slice, target the last element
("Captain Crunch") and replace it with "Lucky Charms".
Print the complete `cereals` array.

Declare a `cookie_crisp` variable that references the
"Cookie Crisp" String.

Declare a `cookie` variable that extracts a slice of
the text "Cookie" from the String. Print the slice.

Declare a `cocoa_puffs` variable. Make it a reference
to the "Cocoa Puffs" String (in other words, a &String).

Declare a `puffs` variable that extracts a slice of
the text "Puffs" from the String. Print the slice.
*/

pub fn project_id140_solution() {
    let mut cereals: [String; 5] = [
        "Cookie Crisp".to_string(),
        "Cinnamon Toast Crunch".to_string(),
        "Frosted Flakes".to_string(),
        "Cocoa Puffs".to_string(),
        "Captain Crunch".to_string(),
    ];

    let first_two = &cereals[..2];
    println!("First two: {:?}", first_two);

    let mid_three = &cereals[1..4];
    println!("Middle three: {:?}", mid_three);

    let last_three = &mut cereals[2..];
    println!("Last three: {:?}", last_three);

    last_three[2] = "Lucky charms".to_string();
    println!("Cereals: {:?}", cereals);

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[..7];
    println!("{cookie}");

    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];
    println!("{puffs}");
}
