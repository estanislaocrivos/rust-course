/* This project corresponds to task number 163 @ course https://www.udemy.com/course/learn-to-code-with-rust/ */

/*
Define a Flight struct with the following fields:
  - an `origin` field (String)
  - a `destination` field (String)
  - a `price` field (f64)
  - a `passengers` field (u32)

Derive a Debug trait implementation for the Flight struct.

Define a `new` constructor function that returns a new
instance of a Flight.

Define a `change_destination` method that accepts a new
destination and overwrites the value of the `destination`
field.

Define a `increase_price` method that raises the value
of the `price` by 20% (multiply the `price` field by 1.20).
Make sure to save the new `price` field value.

Define a `itinerary` method that prints out both the
`origin` and `destination` fields in the following format
(origin -> destination).

Use the constructor function to create a new Flight instance
in the main function. Invoke all of the defined methods.
Print out the struct in Debug format to confirm the struct
updates as you expect.

Use struct update syntax to copy the `price` and `passengers`
fields to a new Flight struct instance. Make sure to provide
new Strings for the remaining fields to ensure ownership
doesn't transfer. Assign the new Flight to a separate variable.
*/

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passenger: u32,
}

const DEFAULT_FLIGHT_ORIGIN: &str = "Argentina";
const DEFAULT_FLIGHT_DESTINATION: &str = "Brazil";
const DEFAULT_FLIGHT_PRICE: f64 = 1.0;
const DEFAULT_FLIGHT_PASSENGER_ID: u32 = 0;

impl Flight {
    fn new() -> Flight {
        return Flight {
            origin: DEFAULT_FLIGHT_ORIGIN.to_string(),
            destination: DEFAULT_FLIGHT_DESTINATION.to_string(),
            price: DEFAULT_FLIGHT_PRICE,
            passenger: DEFAULT_FLIGHT_PASSENGER_ID,
        };
    }

    fn change_destination(&mut self, new_dest: &str) {
        self.destination = new_dest.to_string();
    }

    fn increase_price(&mut self) {
        self.price *= 1.2;
    }

    fn set_passenger_id(&mut self, new_id: u32) {
        self.passenger = new_id;
    }

    fn itinerary(&self) {
        println!(
            "Origin: {} -> Destination: {}",
            self.origin, self.destination
        );
    }
}

pub fn project_id163_solution() {
    let mut flight_1 = Flight::new();
    println!("flight_1: {:?}", flight_1);
    flight_1.change_destination("Colombia");
    flight_1.increase_price();
    println!("flight_1: {:?}", flight_1);
    flight_1.itinerary();

    let mut flight_2 = Flight {
        origin: "Colombia".to_string(),
        destination: "Argentina".to_string(),
        ..flight_1
    };
    flight_2.set_passenger_id(45);
    println!("flight_2: {:?}", flight_2);
}
