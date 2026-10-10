use std::time::Instant;

fn main() {
    let now = Instant::now();
    println!("Time_passed: {:?}", now.elapsed());
}
