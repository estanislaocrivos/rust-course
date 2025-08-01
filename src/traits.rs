// src/traits.rs

pub fn traits_in_rust() {
    struct Human;
    struct Cat;

    /* A trait can be used to define an interface */
    trait Talk {
        fn say_hello(&self) -> String;
    }

    /* Then you can implement the interface for different objects */

    impl Talk for Human {
        fn say_hello(&self) -> String {
            return "Hola!".to_string();
        }
    }

    impl Talk for Cat {
        fn say_hello(&self) -> String {
            return "Miau!".to_string();
        }
    }

    let john = Human;
    println!("John says {}", john.say_hello());

    let chichi = Cat;
    println!("Chichi says {}", chichi.say_hello());
}
