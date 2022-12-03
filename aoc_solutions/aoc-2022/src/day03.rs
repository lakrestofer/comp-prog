use std::collections::HashSet;

pub fn solve_first(input: String) {
    let sum: u32 = input
        .lines()
        .map(|line| find_common_item(line))
        .map(|common_item| priority(common_item))
        .sum();
    println!("{sum}");
}
pub fn solve_second(input: String) {
    let sum: u32 = input
        .lines()
        .array_chunks::<3>()
        .map(|[x, y, z]| find_badge(x, y, z))
        .map(priority)
        .sum();
    println!("{sum}");
}

fn find_badge(x: &str, y: &str, z: &str) -> char {
    let x = x.chars().collect::<HashSet<char>>();
    let y = y.chars().collect::<HashSet<char>>();
    let z = z.chars().collect::<HashSet<char>>();
    let new = x.intersection(&y).map(|c| (*c)).collect::<HashSet<char>>();
    *(new.intersection(&z).next().unwrap())
}

fn find_common_item(line: &str) -> char {
    let len = line.len();
    let (v, h) = line.split_at(len / 2);
    let v_chars = v.chars().collect::<HashSet<char>>();
    let h_chars = h.chars().collect::<HashSet<char>>();
    *v_chars.intersection(&h_chars).next().unwrap()
}

fn priority(c: char) -> u32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}
