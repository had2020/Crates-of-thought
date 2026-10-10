/*
In this kata, you have an input string and you should check whether it is a valid message. To decide that, you need to split the string by the numbers, and then compare the numbers with the number of characters in the following substring.

For example "3hey5hello2hi" should be split into 3, hey, 5, hello, 2, hi and the function should return true, because "hey" is 3 characters, "hello" is 5, and "hi" is 2; as the numbers and the character counts match, the result is true.

 */

fn is_valid_message(msg: &str) -> bool {
    let mut r = true;
    for i in 0..msg.len() - 1 {
        if msg.chars().nth(i).unwrap().is_numeric() {
            let j: usize = msg.chars().nth(i).unwrap().to_string().parse().unwrap();
            if msg.len() > i + j {
                if !msg
                    .to_string()
                    .chars()
                    .nth(i + j as usize)
                    .unwrap()
                    .is_numeric()
                {
                    break;
                } else {
                    r = false;
                }
            }
        }
    }
    r
}

fn main() {
    println!("{}", is_valid_message("3hey5hello2hi"));
}
