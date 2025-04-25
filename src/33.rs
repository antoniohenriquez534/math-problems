use std::cmp;

fn main() {
    let numbers = vec![-2, -1, 0, 1, 2];
    
    if let [min, max] = numbers.iter().enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(i, v)| v.abs())
        .min_max_by_key(|a, b| a.partial_cmp(&b).unwrap())
    {
        println!("The minimum and maximum elements are: {} and {}", min, max);
    } else {
        eprintln!("No numbers to compare.");
    }
}
