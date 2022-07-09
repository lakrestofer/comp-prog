pub fn solve_first(input: String) {
    let mut lines = input.lines().peekable();

    let width = lines.peek().unwrap().len();
    let mut counter = vec![0; width];
    let mut length = 0;

    for line in input.lines() {
        length += 1;
        for (index, char) in line.chars().enumerate() {
            if char == '1' {
                counter[index] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    for (exp, bit_count) in counter.iter().rev().enumerate() {
        if *bit_count > length / 2 {
            gamma += 1 << exp;
        } else {
            epsilon += 1 << exp;
        }
    }
    let consumption_rate = gamma * epsilon;
    println!("rate: {}", consumption_rate);
}
pub fn solve_second(input: String) {
    // TODO continue with https://adventofcode.com/2021/day/3#part2
}
