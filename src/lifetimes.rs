// src/lifetimes.rs

pub fn lifetimes_in_rust() {
    /* Lifetimes express when a reference starts living and when it ends. Lifetimes help the compiler determine the time of survival of references and help the program not crash during execution due to dangling pointers, for example */

    /* The lifetime of a reference ends in the last place where it is used, so the scope where it is used may not be used as a reference for its lifetime */

    let reference = {
        let numbers = [1, 2, 3];
        let slice = &numbers[..1];
        // slice // This generates an error, because you would assign a reference to numbers to the variable reference, but as soon as this scope ends, numbers will not exist anymore, so the reference will be dangling. The error is "numbers does not live long enough" or "slice outlives numbers"
    };

    let numbers = vec![1, 2, 3];
    let nums = numbers; // Move ownership

    // let reference = &numbers; // This will throw an error because ownership was moved

    let reference_1 = &nums[..1];
    let reference_2 = nums;
    // println!("reference_1 = {:?}", reference_1); // This will throw an error because I am moving ownership right after the def. of reference_1. With this line commented, rust "discards" the reference_1 operation because it detects it is not used, so directly transfers ownership to reference_2. I can use reference_2 if I want below

    println!("reference_2 = {:?}", reference_2);
}
