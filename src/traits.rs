// src/traits.rs

pub fn traits_in_rust() {
    /* Traits allow to define interfaces for objects, these interfaces may or may not contain an implementation. It allows for a contract to be stablished for certain functionality without the need of implementation. */

    /* You can define a trait using the trait directive. Inside the trait block you can declare functions/methods signatures. These signatures may contain self as a parameter or not, depending on the context (method or associated function or a simple function) */
    trait Talk {
        fn say_hello(&self) -> String;
    }

    /* Then you can implement the interface for different objects: */
    struct Human;
    struct Cat;

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
    println!("John says: {}", john.say_hello());

    let chichi = Cat;
    println!("Chichi says: {}", chichi.say_hello());

    /* You may also define a default implementation for a trait. Is there is no implementation for the trait, this default is used */
    trait Jump {
        fn jump(&self) -> String {
            return "I am jumping!".to_string();
        }
    }

    impl Jump for Cat {}
    impl Jump for Human {}

    println!("John says: {}", john.jump());
    println!("Chichi says: {}", chichi.jump());

    impl Cat {
        fn say_something(&self) {
            println!("{}, miau miau miau miau...", self.say_hello()); // You may also call traits from inside the methods
        }
    }

    /* You can pass objects which implement certain trait as parameter, for example */
    fn take_one(entity: &impl Talk) {
        println!("The entity says: {}", entity.say_hello());
    }
    take_one(&chichi);
}
