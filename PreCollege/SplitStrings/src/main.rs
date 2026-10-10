/*
Complete the solution so that it splits the string into pairs of two characters. If the string contains an odd number of characters then it should replace the missing second character of the final pair with an underscore ('_').
*/

fn solution(s: &str) -> Vec<String> {
    let mut r: Vec<String> = vec![];
    for i in 0..s.to_string().len() {
        if (i % 2) != 0 {
            r.push(format!(
                "{}{}",
                s.to_string().chars().nth(i - 1).unwrap().to_string(),
                s.to_string().chars().nth(i).unwrap().to_string()
            ));
        }
    }
    if (s.to_string().len() % 2) != 0 {
        r.push(format!(
            "{}_",
            s.to_string()
                .chars()
                .nth(s.to_string().len() - 1)
                .unwrap()
                .to_string()
        ));
    }
    r
}

fn main() {
    println!("{:?}", solution("abcdef"));
}
