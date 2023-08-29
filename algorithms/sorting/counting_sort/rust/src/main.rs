const INPUT: [usize; 20] = [
    18, 4, 3, 16, 1, 14, 5, 6, 19, 10, 9, 15, 12, 20, 17, 7, 13, 11, 2, 8,
];
fn main() {
    let list = Vec::from(INPUT);
    let max: usize = *list.iter().max().unwrap();
    let mut register = vec![0; max + 1];
    let mut output = vec![0; list.len()];
    for item in list {
        register[item] = 1
    }
    for i in 0..max {
        register[i + 1] += register[i]
    }
}
fn str_to_int(s: &str) -> usize {
    let mut sum = 0;
    for (exp, char) in s.chars().rev().enumerate() {
        if char == '1' {
            sum += if exp == 0 { 1 } else { 1 << exp };
        }
    }
    sum
}
