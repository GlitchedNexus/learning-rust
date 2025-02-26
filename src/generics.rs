// Generics give us the ability to reuse code with different data types
// put qnother way, let us reduce code duplication.
pub fn sum_of_elements<T>(nums: &[T]) -> T {
    let mut sum = nums[0];
    for i in 1..nums.len() {
        sum += nums[i];
    }
    sum
}

pub fn run() {
    let 32_bit_nums: [i32; 5] = [1, 2, 3, 4, 5];
    let 64_bit_nums: [i64; 5] = [1, 2, 3, 4, 5];

    println!("Sum of 32 bit nums: {}", sum_of_elements(&[32_bit_nums]));
    println!("Sum of 64 bit nums: {}", sum_of_elements(&[64_bit_nums]));
}