use std::collections::HashMap;

fn count_chars(input: &str) -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();

    for ch in input.chars() {
        if result.contains_key(&ch) {
            result.insert(ch, result.get(&ch).unwrap() + 1 as usize);
        } else {
            result.insert(ch, 1);
        }
    }

    result
}

fn main() {
    let counts = count_chars("hello"); // an input in here
    println!("{:?}", counts);
}
