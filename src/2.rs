use std::ops::Add;
use rand::Rng;

fn generate_random_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}
