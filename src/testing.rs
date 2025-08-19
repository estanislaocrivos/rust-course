// src/testing.rs

fn testable_function(x: u8) -> u8 {
    return x;
}

fn is_it_true(x: u8) -> bool {
    if x > 0 {
        return true;
    } else {
        return false;
    }
}

fn this_function_may_panic(x: u8) {
    if x > 0 {
        panic!("Panicking!");
    } else {
        println!("Not panicking...");
    }
}

fn divide(a: f64, b: f64) -> Result<f64, ()> {
    if b == 0.0 {
        return Err(());
    }
    return Ok(a / b);
}

#[cfg(test)]
mod test {
    use super::*; // Use all functions defined on this file
    use pretty_assertions::assert_eq;

    #[test]
    fn testable_function_test() {
        assert_eq!(testable_function(1), 1);
        assert_eq!(testable_function(2), 2);
        assert_eq!(
            testable_function(3),
            3,
            "This text will only be printed on assertion failure..."
        );
    }

    #[test]
    fn is_it_true_test() {
        assert!(is_it_true(1));
        assert_eq!(is_it_true(0), false);
    }

    #[test]
    #[should_panic] // This is how you tell Rust "check that this function panics"
    fn this_function_may_panic_test_1() {
        this_function_may_panic(1);
    }

    #[test]
    #[should_panic(expected = "Panicking")] // You may also specify which panic you expect to happen based on its panic message
    fn this_function_may_panic_test_2() {
        this_function_may_panic(1);
    }

    #[test]
    fn divide_test() {
        assert_eq!(divide(2.0, 2.0), Ok(1.0));
        assert_eq!(divide(2.0, 0.0), Err(()));
    }

    #[test]
    fn this_test_returns_a_result_enum() -> Result<(), u8> {
        // This test passes because it returns Ok() (no assert used)
        let x: bool = true;
        if x {
            return Ok(());
        } else {
            return Err(20);
        }
    }
}
