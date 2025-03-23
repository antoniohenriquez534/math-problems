fn main() {
    // Example usage of vector and iterator operations
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Sum of all even numbers: {}", sum_even(&numbers));
}

// Function to calculate the sum of even numbers in a given vector
fn sum_even(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for &number in numbers.iter() {
        if number % 2 == 0 {
            total += number;
        }
    }
    total
}

// Helper function to check the correctness of the sum_even function
fn check(numbers: &[i32], expected_sum: i32) -> bool {
    let result = sum_even(&numbers);
    assert_eq!(result, expected_sum, "Expected sum {} but got {}", expected_sum, result);
    true
}

// Example usage and verification with a provided solution
let input_numbers = vec![10, 15, 20, 30, 35];
let expected_result = 80;
check(&input_numbers, expected_result);
