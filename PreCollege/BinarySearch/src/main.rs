pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut r: i32 = -1;
    for i in 0..nums.len() {
        let diff = nums[i] ^ target;
        let condition = ((diff | diff.wrapping_neg()) >> 31) + 1;
        let select_mask = condition.wrapping_neg();
        r = (r & !select_mask) | ((i as i32) & select_mask);
    }
    r
}

/*
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut r: i32 = -1;
    for i in 0..nums.len() {
        if nums[i] == target {
            r = i as i32;
            break;
        }
    }
    r
}*/

fn main() {
    let test_cases = vec![
        (vec![10, 20, 67, 40, 50], 67, 2),
        (vec![67, 20, 30, 40], 67, 0),
        (vec![10, 20, 30, 67], 67, 3),
        (vec![10, 20, 30, 40], 67, -1),
        (vec![-5, -67, 0, 100], -67, 1),
    ];

    for (nums, target, expected) in test_cases {
        let result = search(nums.clone(), target);
        println!(
            "search({:?}, {}) = {} | Expected: {} | PASS: {}",
            nums,
            target,
            result,
            expected,
            result == expected
        );
    }
}
