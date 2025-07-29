/* This project corresponds to task number 117 @ course https://www.udemy.com/course/learn-to-code-with-rust/ */

/*
Declare a `is_concert` variable set to a boolean.
Declare a `is_event` variable assigned to `is_concert`.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

No ownership move. Both variables hold the same value, each one with dedicated memory on the stack.

Declare a `sushi` variable to set to a string literal of "Salmon"
Declare a `dinner` variable assigned to the `sushi` variable.
Will Rust move ownership? State your answer, then confirm
by trying to printing both variables out.

String literals live on flash memory, in this case both variables point to the begining of the string literal on flash, no ownership move.

Repeat the previous example but use a heap String instead.
Will Rust move ownership? Explain why the result is different
from the previous operation.

With heap-allocated strings there is a movement of ownership. When you move a String, only the new variable can manipulate it.

The `clear` method modifies a heap String to have no content.
Declare an `eat_meal` function that accepts a `meal` parameter
of type String. In the body of `eat_meal`, invoke the `clear`
method on the `meal` parameter.

In the `main` function, invoke the `eat_meal` function and pass
in your "Salmon" String. Explain what happens when the eat_meal
function runs. Describe the complete movement of ownership of
the "Salmon" String throughout the program.

Say we want to keep the String around after `eat_meal` is
called. How can we continue to have access to the String in
the `main` function? Print out the (empty) String.
*/

pub fn eat_meal(mut meal: String) -> String {
    meal.clear();
    return meal;
}

/* Using references */
pub fn eat_meal_ref(meal: &mut String) {
    meal.clear();
}
