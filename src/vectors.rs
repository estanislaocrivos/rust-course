// src/vectors.rs

pub fn vectors_in_rust() {
    /* Vectors are similar to arrays, but just like heap-allocated strings, they have the ability to grow during execution, heap-allocating elements */

    let mut vector_1: Vec<i32> = Vec::new(); // New vector of i32 elements
    vector_1.push(32);

    let mut vector_2 = vec![1, 2, 4]; // With the vec! macro you avoid having to declare the vector elements's type
    vector_2.push(5);
    vector_2.insert(2, 3);
    let element = vector_2.pop(); // Returns an optional
    vector_2.remove(0); // Removes element at index 0 & shifts elements. If element did not exist, it panicks
    println!("vector_2 = {:?}", vector_2);
    println!("vector_2[0] = {}", vector_2[0]); // You access vector elements just like array elements

    let vector_2_slice = &vector_2[..2];

    /* The program will panick at runtime if we try to access an invalid index position. It cannot detect these kind of operations @ compile time due to Rust not knowing the actual vector size. In this case it is more convenient to use the get() method, which returns an optional */

    let index = 20;
    let option = vector_2.get(index);
    match option {
        Some(element) => println!("The element @ index {index} is: {}", element),
        None => println!("The element @ index {index} does not exist!"),
    }

    let mut vector3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vector3_ref = &mut vector3;
    vector3_ref.push(10); // Modifying a vector through a mutable reference
    println!("vector_3 = {:?}", vector3);
}
