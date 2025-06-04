fn main() {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
