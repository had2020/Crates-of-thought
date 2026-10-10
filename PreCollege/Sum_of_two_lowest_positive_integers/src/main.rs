// https://www.codewars.com/kata/558fc85d8fd1938afb000014/rust
/*
Create a function that returns the sum of the two lowest positive numbers given an array of minimum 4 positive integers.
No floats or non-positive integers will be passed.

For example, when an array is passed like [19, 5, 42, 2, 77], the output should be 7.

[10, 343445353, 3453445, 3453545353453] should return 3453455.
 */

fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    let mut s1: &u32 = &std::u32::MAX;
    let mut s2: &u32 = &std::u32::MAX;
    for i in numbers {
        if s1 > i && s1 > s2 {
            s1 = i;
        } else if s2 > i {
            s2 = i;
        }
    }
    s1 + s2
}

fn main() {
    println!("{}", sum_two_smallest_numbers(&[3, 1, 4, 23])); // will print 4, because 3+1=4
}
