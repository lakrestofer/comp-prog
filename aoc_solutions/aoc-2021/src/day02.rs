use crate::utils::fastParse;

pub fn solve_first(input: String) {
    let mut h = 0;
    let mut v = 0;

    for line in input.lines() {
        if line.starts_with("forward") {
            h += fastParse::<u32>(line.split_at(8).1);
        } else if line.starts_with("down") {
            v += fastParse::<u32>(line.split_at(5).1)
        } else if line.starts_with("up") {
            v -= fastParse::<u32>(line.split_at(3).1)
        } // else do noth
    }

    let result = h * v;
    println!("{result}");
}
pub fn solve_second(input: String) {
    let mut h = 0;
    let mut v = 0;
    let mut aim = 0;

    for line in input.lines() {
        if line.starts_with("forward") {
            let amount = fastParse::<u32>(line.split_at(8).1);
            h += amount;
            v += aim * amount;
        } else if line.starts_with("down") {
            aim += fastParse::<u32>(line.split_at(5).1)
        } else if line.starts_with("up") {
            aim -= fastParse::<u32>(line.split_at(3).1)
        } // else do noth
    }

    let result = h * v;
    println!("{result}");
}
