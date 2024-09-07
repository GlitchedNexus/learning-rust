// Vectors - Resizable arrays

use std::mem;

pub fn run() {

    // Defining an array
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    // Adding items into a vector

    numbers.push(6);
    numbers.push(7);
    println!("{:?}", numbers);

    // Pop last element
    numbers.pop();
    println!("{:?}", numbers);

    // Accesing an index
    println!("First element: {}", numbers[0]);

    // Re-assigning values
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Getting length of array
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Looping through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Looping & mutating values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Updated Vector: {:?}", numbers);
}