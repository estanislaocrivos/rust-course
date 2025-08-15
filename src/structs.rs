// src/structs.rs

pub fn structs_in_rust() {
    #[derive(Debug)] // This allows us to use the debug prints on structures
    struct Coffee {
        coffee_type: String,
        is_cold: bool,
        has_milk: bool,
        size_ml: i32,
    }

    const DEFAULT_COFFEE_TYPE: &str = "moka";
    const DEFAULT_COFFEE_SIZE_ML: i32 = 50;

    /* Methods on Coffee class */
    impl Coffee {
        /*
        If the method to be implemented needs to access the structure (mut or immut) the first parameter must be self. Available options are:

        - self (immutable, by parameter)
        - mut self (mutable, by parameter)
        - &self (immutable reference)
        - &mut self (mutable reference)

        The last two are the most used because in both cases you end up implicitly passing a reference to the object so there is no change in ownership.

        If a method does not take self as parameter, the function is referred to as an associated function, which are usually used for constructors/factories, etc. (the new() function below is an example of an associated function).
        */

        fn new() -> Coffee {
            // This can be thought as the object constructor
            return Coffee {
                coffee_type: DEFAULT_COFFEE_TYPE.to_string(),
                is_cold: false,
                has_milk: false,
                size_ml: DEFAULT_COFFEE_SIZE_ML,
            };
        }

        fn print_type(&self) {
            // We could use Coffee instead of Self, it is the same (Rust knows Self is Coffee because this is an implementation block for Coffee)
            println!("Coffee type: {}", self.coffee_type);
        }

        fn print_size(&self) {
            println!("Coffee size: {} ml", self.size_ml);
            self.print_type(); // You can also call other implemented methods from within a method
        }

        fn hot_to_cold(&mut self) {
            self.is_cold = true;
        }

        fn is_better_than(&self, other_coffee: &Coffee) {
            println!(
                "Coffee of type {} is better than the other coffee of type {}",
                self.coffee_type, other_coffee.coffee_type
            );
        }
    }

    /* Rust allows to have multiple implementation blocks for the same "class". It will automatically merge all the implementations blocks in one on compile time */
    impl Coffee {
        fn new_with_params(
            coffee_type: String,
            is_cold: bool,
            has_milk: bool,
            size_ml: i32,
        ) -> Coffee {
            return Coffee {
                // Automatic assignment due to match between function params. and struct members
                coffee_type,
                is_cold,
                has_milk,
                size_ml,
            };
        }
    }

    let coffee = Coffee {
        // You can create a new object "by hand"
        coffee_type: "espresso".to_string(),
        is_cold: false,
        has_milk: false,
        size_ml: 32,
    };

    println!("The coffee object is: {:?}", coffee); // This is allowed thanks to the derived Debug trait

    let coffee = Coffee::new(); // Or using the implementation of the constructor (new() method)

    let mut coffee = Coffee::new_with_params("moka".to_string(), false, false, 20); // Or using the other constructor implementation

    // let coffee_type = coffee.coffee_type; // Moving ownership, be careful!

    println!("Is the coffee cold? {}", coffee.is_cold);
    coffee.hot_to_cold();
    println!("Now, is the coffee cold? {}", coffee.is_cold);

    fn modify_coffee_type(mut coffee: Coffee, new_type: &str) -> Coffee {
        coffee.coffee_type = new_type.to_string();
        return coffee;
    }

    fn modify_coffee_type_by_ref(coffee: &mut Coffee, new_type: &str) {
        coffee.coffee_type = new_type.to_string();
    }

    println!("The coffee type is: {}", coffee.coffee_type);
    coffee = modify_coffee_type(coffee, "new coffee type, by parameter");
    println!("The coffee type is: {}", coffee.coffee_type);
    modify_coffee_type_by_ref(&mut coffee, "another coffee type, by reference");
    println!("The coffee type is: {}", coffee.coffee_type);

    coffee.print_size();

    let mut coffee_1 = Coffee::new();
    coffee_1.coffee_type = "moka".to_string();
    let mut coffee_2 = Coffee::new();
    coffee_2.coffee_type = "latte".to_string();
    coffee_1.is_better_than(&coffee_2);

    /* Builder pattern: returning a mutable ref. to self from the object's methods allow chaining actions. Example below */

    impl Coffee {
        fn change_size(&mut self, size: i32) -> &mut Self {
            self.size_ml = size;
            return self;
        }
        fn add_milk(&mut self) -> &mut Self {
            self.has_milk = true;
            return self;
        }
    }

    coffee.change_size(100).add_milk(); // You can chain methods

    /* Another example */

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
        };
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

    /* Tuple structs: these kind of structs allow to hold multiple parameters but in a tuple-like style, with no "tags" associated to each element. This allows to implement type-specific tuples, unlike generic tuples */

    struct ShortDuration(u32, u32); // Hours, minutes
    struct LongDuration(u32, u32); // Years, months

    let some_time = ShortDuration(4, 5);
    println!("some_time, h: {}, m: {}", some_time.0, some_time.1);

    // let some_time = (4, 5); // This is a common touple and does not have a specific type, unlike a struct tuple

    /* Unit type structs: these kind of structs allow to hold no members of data, but may have an implementation block associated */

    struct EmptyStruct;

    impl EmptyStruct {
        fn empty_method() {} // Returns ()
    }
}
