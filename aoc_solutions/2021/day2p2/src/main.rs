fn main() {
    let input = std::fs::read_to_string("input.txt").expect("could not read from inputn");

    let mut horizontal: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;
    for line in input.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        match split.as_slice() {
            &[direction, amount] => {
               let amount_n: i64 = amount.parse().unwrap();
               match direction {
                   "forward" => {
                       horizontal += amount_n;
                       depth += (aim * amount_n)
                   },
                   "down" => aim += amount_n,
                   "up" => aim -= amount_n,
                   _ => unreachable!(),
               }
           }
           _ => unreachable!(),
       }
    }
    println!("{}", horizontal * depth);
}
