use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();

    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    while let Some(front) = queue.pop_front() {
        println!("Dequeued: {}", front);
    }
}
