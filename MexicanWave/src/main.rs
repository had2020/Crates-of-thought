/*
Task
In this simple Kata your task is to create a function that turns a string into a Mexican Wave. You will be passed a string and you must return an array of strings where an uppercase letter is a person standing up.

Rules
1.  The input string will always consist of lowercase letters and spaces, but may be empty, in which case you must return an empty array. 2.  If the character in the string is whitespace then pass over it as if it was an empty seat
 */

fn wave(s: &str) -> Vec<String> {
    let mut r: Vec<String> = Vec::new();
    for i in 0..s.len() {
        let mut f: String = String::new();
        for j in 0..s.len() {
            if i == j {
                f.push_str(&(s.chars().nth(j).unwrap().to_uppercase().to_string()));
            } else {
                f.push_str(&(s.chars().nth(j).unwrap().to_string()));
            }
        }
        if s.chars().nth(i).unwrap() != ' ' {
            r.push(f);
        }
    }
    r
}

fn main() {
    println!("{:?}", wave("hello"));
}
