/*
There is a queue for the self-checkout tills at the supermarket. Your task is write a function to calculate the total time required for all the customers to check out!

input
customers: an array of positive integers representing the queue. Each integer represents a customer, and its value is the amount of time they require to check out.
n: a positive integer, the number of checkout tills.
output
The function should return an integer, the total time required.
*/

fn queue_time(customers: &[u32], n: u32) -> u32 {
    let mut cus: Vec<u32> = customers.to_vec();
    let mut iter = 0;
    loop {
        if !cus.is_empty() {
            for i in 0..n {
                if cus.len() > i as usize {
                    if cus[i as usize] == 0 {
                        cus.remove(i as usize);
                    } else {
                        cus[i as usize] -= 1;
                    }
                }
            }
        } else {
            break;
        }
        iter += 1;
    }
    iter
}

fn main() {
    println!("{}", queue_time(&[], 1)); // should be 0
    println!("{}", queue_time(&[5], 1)); // 5
    println!("{}", queue_time(&[2], 5)); // 2
    println!("{}", queue_time(&[1, 2, 3, 4, 5], 1)); // 15
    println!("{}", queue_time(&[1, 2, 3, 4, 5], 100)); // 5
    println!("{}", queue_time(&[2, 2, 3, 3, 4, 4], 2)); // 9
}
