use input_method::input;
use std::collections::HashMap;

fn main() {
    // init
    let mut map = HashMap::from([("Sam", 9), ("Mark", 2), ("Ben", 30)]);

    // iterating
    for key in map.keys() {
        println!("{key}");
    }

    // inserting
    map.insert("John", 45);

    println!("Who do you want to find?");
    let person: String = input();

    // finding
    if let Some(key) = map.get(person.as_str()) {
        println!("key: {}", key);
    } else {
        println!("Key not found!");
    }

    // updating
    map.entry("John").or_insert(88);

    // removing
    map.remove("Sam");
}
