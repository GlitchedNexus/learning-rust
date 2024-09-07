// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {

    // Defining an array
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers);

    // Accesing an index
    println!("First element: {}", numbers[0]);

    // Re-assigning values
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Getting length of array
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers;
    println!("Slice: {:?}", slice);

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}