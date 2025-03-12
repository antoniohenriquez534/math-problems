fn main() {
    let num1 = rand::thread_rng().gen_range(0..10);
    let num2 = rand::thread_rng().gen_range(0..10);
    println!("{} + {} =", num1, num2);
}
