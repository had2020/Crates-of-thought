/*
https://www.codewars.com/kata/54a91a4883a7de5d7800009c/train/rust

Your job is to write a function which increments a string, to create a new string.

If the string already ends with a number, the number should be incremented by 1.
If the string does not end with a number. the number 1 should be appended to the new string.
Examples:

foo -> foo1

foobar23 -> foobar24

foo0042 -> foo0043

foo9 -> foo10

foo099 -> foo100

Attention: If the number has leading zeros the amount of digits should be considered.
 */

fn increment_string(s: &str) -> String {
    let mut r: String = String::new();
    let mut num: String = String::new();
    let mut zeros: String = String::new();
    for i in (0..s.chars().count()).rev() {
        if s.chars().nth(i).unwrap().is_ascii_digit() {
            if !(num != String::new() && s.chars().nth(i).unwrap() == '0') {
                num = format!("{}{}", s.chars().nth(i).unwrap(), num);
            } else {
                zeros = format!("{}0", zeros);
            }
        } else {
            break;
        }
    }
    for i in 0..s.chars().count() {
        if !s.chars().nth(i).unwrap().is_ascii_digit() {
            r = format!("{}{}", r, s.chars().nth(i).unwrap())
        }
    }
    if num != String::new() {
        let nn: usize = num.parse::<usize>().unwrap() + 1;
        if format!("{}", nn).chars().count() != format!("{}", num).chars().count()
            && zeros != String::new()
        {
            zeros.remove(0);
        }
        format!("{}{}{}", r, zeros, nn)
    } else {
        format!("{}1", r)
    }
}

fn main() {
    increment_string("foobar099");
}
