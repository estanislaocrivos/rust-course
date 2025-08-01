// src/vectors.rs

pub fn vectors_in_rust() {
    /* Vectors are similar to arrays, but just like heap-allocated strings, they have the ability to grow during execution, heap-allocating elements */

    let mut vector: Vec<i32> = Vec::new(); // New vector of i32 elements
    vector.extend([1, 2, 3, 4, 5, 6, 7, 8, 9]); // Pushes an iterable
    vector.push(10); // Pushes a new element to the end of the vector
    vector.insert(0, 0); // Insert an element in a certain index
    vector.remove(0); // Removes element at index & shifts elements. If element did not exist, it will panic

    let element = vector.pop(); // Pops the last element, returns an optional
    let element = vector[0]; // You can access the vector elements by index, but this can panic if the element @ index does not exist
    let element = vector.get(0); // Gets an element @ index. Returns an optional, None or element (this avoids panicking)
    println!("vector = {:?}", vector);

    /* The program will panic at runtime if we try to access an invalid index position. It cannot detect these kind of operations @ compile time due to Rust not knowing the actual vector size. In this case it is more convenient to use the get() method, which returns an optional */
}
