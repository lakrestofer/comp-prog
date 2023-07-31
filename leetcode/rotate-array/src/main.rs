pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let l = nums.len();
    let k = k as usize;
    let mut new = vec![0; l];
    let mappings = (0..l).map(|i| (i + k) % l).enumerate();
    for (old_i, new_i) in mappings {
        new[new_i] = nums[old_i]
    }
    nums.copy_from_slice(&new)
}

fn main() {
    let mut input = vec![1, 2, 3, 4];
    let k = 7;
    rotate(&mut input, k);
    println!("after: {:?}", input);
}
