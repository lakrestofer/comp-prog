use std::collections::HashSet;

pub fn solve_first(input: String) {
    let sum: usize = input.lines().map(|line| find_marker(line, 4)).sum();
    println!("{sum}");
}
pub fn solve_second(input: String) {
    let sum: usize = input.lines().map(|line| find_marker(line, 14)).sum();
    println!("{sum}");
}

fn find_marker(line: &str, size: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();

    let n: usize = chars
        .windows(size)
        .enumerate()
        .find(|(_, w)| is_marker(w))
        .map(|(i, _)| i + size)
        .unwrap();
    n
}

fn is_marker(chars: &[char]) -> bool {
    let len = chars.len();
    let unique_chars = chars.into_iter().collect::<HashSet<&char>>();
    let unique_len = unique_chars.len();
    len == unique_len
}

fn char_to_prime(c: char) -> usize {
    match c {
        'a' => 2,
        'b' => 3,
        'c' => 5,
        'd' => 7,
        'e' => 11,
        'f' => 13,
        'g' => 17,
        'h' => 19,
        'i' => 23,
        'j' => 29,
        'k' => 31,
        'l' => 37,
        'm' => 41,
        'n' => 43,
        'o' => 47,
        'p' => 53,
        'q' => 59,
        'r' => 61,
        's' => 67,
        't' => 71,
        'u' => 73,
        'v' => 79,
        'w' => 83,
        'x' => 89,
        'y' => 97,
        'z' => 101,
        _ => 1,
    }
}
