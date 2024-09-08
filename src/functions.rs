// Functions - Used to store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Jane");

    // We can bind function values to variables.
    let sum = add(5, 5);
    println!("Sum: {}", sum);

    // Closure - allow access to variables that may be 
    //           blocked due to function domain.
    let n3 = 12;
    let add_nums = |n1:i32, n2:i32| n1 + n2 +n3;
    println!("Closure sum: {}", add_nums(3,3));

}

fn greeting(greet: &str, name:&str) {
    println!("{} {}. Nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    // Lack of semicolon tells rust that it returns this value.
    // n1 + n2
    return n1 + n2;
}