/*
A pangram is a sentence that contains every single letter of the alphabet at least once. For example, the sentence "The quick brown fox jumps over the lazy dog" is a pangram, because it uses the letters A-Z at least once (case is irrelevant).

Given a string, detect whether or not it is a pangram. Return True if it is, False if not. Ignore numbers and punctuation.
 */

fn is_pangram(s: &str) -> bool {
    let a = "abcdefghijklmnopqrstuvwxyz";
    let mut t: [bool; 26] = [false; 26];
    for i in 0..s.chars().count() {
        for j in 0..a.chars().count() {
            if s.chars().nth(i).unwrap().to_lowercase().to_string()
                == a.chars().nth(j).unwrap().to_lowercase().to_string()
            {
                t[j] = true;
            }
        }
    }
    let mut r = true;
    for i in 0..t.len() {
        if t[i] == false {
            r = false;
            break;
        }
    }
    r
}

fn main() {
    println!(
        "{}",
        is_pangram("The quick, brown fox jumps over the lazy dog!")
    )
}
