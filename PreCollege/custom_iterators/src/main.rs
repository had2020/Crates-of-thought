fn main() {
    let nums = vec![303, 404, 202];
    nums.iter().for_each(|i| println!("{i}"));

    let nums1 = [1, 2, 3];
    let mapped_nums1 = nums1.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("mapped_nums1: {:?}", mapped_nums1);
}
