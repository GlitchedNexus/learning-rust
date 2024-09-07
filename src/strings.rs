// Primitive str = Immutable fixed-length string somewhere in memory
// 
// String = Growable, heap-allocated data structure - Use when you need 
// to modify or own string data

pub fn run() {

    //  Immutable fixed-length string
    let hello = "Hello";
    println!("{}", hello);
    println!("Length: {}", hello.len());

    // Growable, heap-allocated data string
    let mut hi = String::from("Hi");

    println!("{}", hi);
    println!("Length: {}", hello.len());

    // Adding chars to the end of the string
    hi.push('W');

    // Adding multiple characters to the a string
    hi.push_str("orld");

    // Checking the capacity in bytes
    println!("Capacity: {}", hi.capacity());

    // Checking if the string is empty
    println!("Is Empty: {}", hi.is_empty());

    // Checking if it contains a given substring
    println!("Contains 'World': {}", hi.contains("World"));

    // Replacing a substring inside a string
    println!("Replace 'World' with 'Planet': {}", hi.replace("World", "Planet"));

    // Loopomg through string using whitespace
    for word in hi.split_whitespace() {
        println!("{}", word);
    }
    
    // Creating strings with a given capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);


    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());


}