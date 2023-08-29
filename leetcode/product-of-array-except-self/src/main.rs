pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut answers = vec![1; nums.len()];

    answers
}

fn main() {
    let input = vec![1, 2, 3, 4];
    let output = vec![24, 12, 8, 6];
    let res = product_except_self(input);
    println!("Result is equal to expected: {}", input == output);
}
