// Generates a random Rust code snippet
fn main() {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen();
    let y: i32 = rng.gen();
    println!("{} + {} = {}", x, y, x + y);
}
