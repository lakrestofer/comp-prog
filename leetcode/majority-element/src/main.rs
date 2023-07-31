pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut count = 1;
    let mut majority = nums[0];

    for val in nums.iter().skip(1) {
        if *val == majority {
            count += 1;
        } else {
            if count == 0 {
                majority = *val;
                count = 1;
            } else {
                count -= 1;
            }
        }
    }
    majority
}

fn main() {
    let input = vec![2, 2, 1, 1, 1, 2, 2];
    let maj = majority_element(input);
    println!("Maj: {maj}");
}
