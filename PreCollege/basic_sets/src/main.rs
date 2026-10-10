use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();

    set.insert("apple");
    set.insert("banana");

    println!("Has apple? {}", set.contains("apple"));
    set.remove("banana");

    for fruit in &set {
        println!("Fruit: {}", fruit);
    }
}
