// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    // The `let` keyword is similar to the `const`
    // keyword in JavaScript
    let name = "Glitched";
    let mut age = 100;

    println!(
        "My name is {} and I am {} years old.", 
        name, age
    );

    // The following line will give us an error
    // if we did not us the `mut` keyword
    // when initialising the age variable.

    age = 200;

    println!(
        "My name is {} and I am {} years ol now.", 
        name, age
    );

    // Define Constants
    const ID: i32 = 001;

    println!("ID: {}", ID); 


    // Assigning multiple variables at once
    let (my_name, my_age) = ("Glitched", 100);

    println!(
        "My name is {} and I am {} years old.", 
        my_name, my_age
    );
}