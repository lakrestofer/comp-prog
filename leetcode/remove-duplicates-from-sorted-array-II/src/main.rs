pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut upper = nums.len();
    let mut k = 0;
    let mut w = 0;
    let mut r = 0;
    let mut p = i32::MAX;
    let mut count = 1;
    while r < upper {
        let c = nums[r];
        if (c != p) {
            nums[w] = c;
            p = c;
            count = 1;
            w += 1;
            k += 1;
            r += 1;
        } else {
            if count == 1 {
                // as long as we've only written the current numbe once we can write it again
                nums[w] = c;
                count = 2;
                w += 1;
                k += 1;
                r += 1;
            } else {
                r += 1;
            }
        }
    }
    k
}

fn main() {
    let mut input = vec![1, 1, 1, 2, 2, 3];
    let k = remove_duplicates(&mut input);
    println!("after: {:?}", input);
    println!("k: {}", k);
}
