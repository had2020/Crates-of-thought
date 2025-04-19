# 🧠 Count Character Frequencies using HashMap
Write a function that takes a &str and returns a HashMap<char, usize> mapping each character to the number of times it appears.

Requirements:
 - Use idiomatic Rust.
 - Avoid unnecessary clones.
 - Optimize for readability and safety.
 - Don’t use any external crates.

✅ Example
``` rust
let input = "hello";
let result = count_chars(input);
assert_eq!(result.get(&'l'), Some(&2));
```
