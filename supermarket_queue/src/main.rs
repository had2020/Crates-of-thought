fn queue_time(customers: &[u32], n: u32) -> u32 {
    if !customers.is_empty() {
        let mut waiting: Vec<u32> = customers.to_vec();
        let mut iterations: u32 = 0;
        let mut in_till: Vec<u32> = Vec::new();
        let mut empty_tills: u32 = n;
        loop {
            if waiting.is_empty() && in_till.is_empty() {
                break;
            }
            if !waiting.is_empty() {
                if empty_tills == 0 {
                    empty_tills = n;
                }
                for i in 0..empty_tills {
                    if (waiting.len() as u32) > (i) {
                        in_till.push(waiting[i as usize]);
                        waiting.remove(i as usize);
                    }
                }
                for i in 0..waiting.len() {
                    waiting[i] += 1;
                }
            }
            for i in 0..in_till.len() {
                if i < in_till.len() {
                    if in_till[i] == 1 {
                        in_till.remove(i);
                        empty_tills += 1;
                    } else {
                        in_till[i] -= 1;
                    }
                }
            }
            iterations += 1;
        }
        iterations
    } else {
        0
    }
}

fn main() {
    println!("{}", queue_time(&[2, 2, 3, 3, 4, 4], 2));
}
