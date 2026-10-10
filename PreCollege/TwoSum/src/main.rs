/*
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut rhs: i32 = i32::MIN;
    let mut lhs: i32 = i32::MIN;
    let mut rhs_iter = 0_usize;
    let mut lhs_iter = nums.len();

    for i in 0..nums.len() {
        rhs_iter += 1;
        lhs_iter -= 1;
    }

    vec![rhs, lhs]
} */
//https://leetcode.com/u/Hadrianusa/
//https://leetcode.com/submissions/detail/2093222613/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut rhs: i32 = i32::MIN;

    let mut nums_clone = nums.clone();

    loop {
        let rhs_idx = nums_clone.len() - 1;
        rhs = nums_clone[rhs_idx];
        nums_clone.pop();
        for i in 0..nums_clone.len() {
            if (rhs + nums_clone[i]) == target {
                return vec![rhs_idx as i32, i as i32];
            }
        }
    }
}

fn main() {
    println!("...");
}
