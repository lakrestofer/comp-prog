pub fn can_jump(jumps_array: Vec<i32>) -> bool {
    let mut max_reach: usize = 0;
    let mut i = 0;
    let n = jumps_array.len();
    while i < n {
        if max_reach < i {
            return false;
        }
        max_reach = max_reach.max(i + jumps_array[i] as usize);
        i += 1;
    }
    return true;
}

fn main() {
    let input = vec![2, 3, 1, 1, 4];
    // let input = vec![3, 2, 1, 0, 4]; // you cannot
    println!(
        "can you jump? {}",
        if can_jump(input) {
            "yes I can"
        } else {
            "no I can't"
        }
    )
}
