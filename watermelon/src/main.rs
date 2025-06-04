// 2 people want to divide a watermelon but want even slices if w is odd output is NO if even YES

fn is_even(w: usize) -> bool {
    if w % 2 == 0 && w > 2 {
        true
    } else {
        false
    }
}

fn main() {
    println!("{}", is_even(3))
}
