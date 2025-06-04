use input_method::input;
use std::collections::HashMap;

fn main() {
    // init
    let mut map = HashMap::from([("sam", 9), ("mark", 2), ("ben", 30)]);

    //iterating
    for key in map.keys() {
        println!("{key}");
    }

    //inserting
    map.insert("john", 45);

    println!("Who do you want to find?");
    let person: String = input();

    //finding
    if let Some(key) = map.get(person.as_str()) {
        println!("key: {}", key);
    } else {
        println!("Key not found!");
    }
}
