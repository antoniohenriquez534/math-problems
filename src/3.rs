fn main() {
    let mut rng = rand::thread_rng();
    let problem = rng.gen_range(1..=10);
    println!("{}", problem);
}
