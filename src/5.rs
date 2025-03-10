use std::ops::{Add, Mul};

struct MathProblem {
    pub a: i32,
    pub b: i32,
}

impl MathProblem {
    fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    fn solve(&self) -> i32 {
        self.a + self.b
    }
}
