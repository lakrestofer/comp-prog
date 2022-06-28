use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("could not read to string");
    let mut lines = buffer.lines();

    let lx: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut l = lx[0];
    let _x = lx[1];

    let mut n_denied = 0;

    for line in lines {
        let (direction, amount_str) = line.split_at(5);
        let (direction, amount): (&str, u32) = (direction, amount_str.trim().parse().unwrap());
        match direction {
            "enter" => {
                if l >= amount {
                    l -= amount
                } else {
                    n_denied += 1
                }
            }
            "leave" => l += amount,
            _ => {}
        }
    }
    println!("{}", n_denied);
}
