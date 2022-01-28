use std::fs;
fn main() {
    let content = fs::read_to_string("input.txt").expect("could not read input from file");

    let lines = content.lines();

    let mut times = 0;

    let mut previous: Option<i64> = None;
    for line in lines {
        match previous {
            Some(value) => {
                let new_number: i64 = line.parse().expect("could not parse line as number");
                if new_number > value {
                    times += 1;
                }
                previous = Some(new_number);
            }
            None => {
                let new_number: i64 = line.parse().expect("could not parse line as number");
                previous = Some(new_number);
            }
        }
    }
    println!("{}", times);
}
