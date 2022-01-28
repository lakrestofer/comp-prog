fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("could not read input");

    let mut horizontal = 0;
    let mut depth = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        match split.as_slice() {
             &[direction, amount] => {
                let amount_n: i64 = amount.parse().unwrap();
                match direction {
                    "forward" => horizontal += amount_n,
                    "down" => depth += amount_n,
                    "up" => depth -= amount_n,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    let computation = horizontal * depth;
    println!("{}", computation);
}
