// Problem 1: Calculate the sum of the first n natural numbers.
fn main() {
    let n = 10;
    println!("The sum of the first {} natural numbers is {}", n, calculate_sum(n));
}

// Function to calculate the sum of the first n natural numbers.
fn calculate_sum(n: u32) -> u32 {
    (n * (n + 1)) / 2
}
