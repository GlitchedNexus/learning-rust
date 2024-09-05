/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of 
// all variables at compile time, however, the compiler can usually infer what 
// type we want to use based on the value and how we use it.

// Convention for naming vars uses `_`

pub fn run() {

    // Integers by default are i32
    let x = 1;

    // Floats by default are f64
    let y = 2.5;

    // Declaring Explicit Types
    let z: i64 = 1;

    // Finding Max Size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Finding Min Size
    println!("Min i32: {}", std::i32::MIN);
    println!("Min i64: {}", std::i64::MIN);

    // Boolean Values
    let is_active = true;

    // Boolean Expressions
    let is_greater = 1 > 5;

    // Chars - note the use of single quotes
    let c = 'c';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, c, face));
}