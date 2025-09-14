fn alphanumeric(password: &str) -> bool {
    let mut r = true;
    if password == "" {
        r = false;
    }
    for i in password.chars() {
        if !((i.is_numeric() || i.is_ascii_alphabetic()) && i != ' ' && i != '_') {
            r = false;
        }
    }
    r
}

fn main() {
    println!("vaild: {}", alphanumeric("passwordhere"));
}

/*
https://www.codewars.com/kata/526dbd6c8c0eb53254000110/rust

In this example you have to validate if a user input string is alphanumeric. The given string is not nil/null/NULL/None, so you don't have to check that.

The string has the following conditions to be alphanumeric:

At least one character ("" is not valid)
Allowed characters are uppercase / lowercase latin letters and digits from 0 to 9
No whitespaces / underscore
 */
