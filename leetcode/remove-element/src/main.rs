pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.len() == 0 {
        return 0;
    };
    let mut buffer = [0; 100];
    let mut i = 0;
    for num in nums.iter() {
        if *num != val {
            buffer[i] = *num;
            i += 1;
        }
    }
    for j in 0..i {
        nums[j] = buffer[j];
    }
    i as i32
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    remove_element(&mut nums, val);
    println!("After: {:?}", nums);
}
