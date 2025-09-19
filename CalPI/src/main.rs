use std::time::{Duration, Instant};

fn pi_leibniz(iterations: u64) -> f64 {
    let mut pi = 0.0;
    for k in 0..iterations {
        let term = (-1.0f64).powi(k as i32) / (2 * k + 1) as f64;
        pi += term;
    }
    4.0 * pi
}

fn main() {
    let start = Instant::now();
    println!(
        "elapsed:{:?}, pi:{}",
        start.elapsed(),
        pi_leibniz(1_000_000)
    );
}
