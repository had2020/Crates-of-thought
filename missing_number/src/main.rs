fn main() {
    let list = vec![2, 3, 1, 5];

    let n: i32 = list.len() as i32 + 1; // because one number is missing
    let expected_sum: i32 = n * (n + 1) / 2;

    let actual_sum: i32 = list.iter().sum();

    println!("Missing number: {}", expected_sum - actual_sum);
}
