pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    let mut unique = 0;
    // num will be in bound -100 to 100
    let mut last_num = -101;
    let mut reader_i = 0;
    let mut writer_i = 0;
    while reader_i < len {
        let num = nums[reader_i];
        if num == last_num {
            // not unique
            reader_i += 1;
            continue;
        } else {
            unique += 1;
            last_num = num;
            nums[writer_i] = num;
            reader_i += 1;
            writer_i += 1;
        }
    }
    return unique;
}
fn main() {
    let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    remove_duplicates(&mut input);
    println!("after: {:?}", input);
}
