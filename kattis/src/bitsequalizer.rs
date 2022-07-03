use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();

    let C = lines.next().unwrap().parse().unwrap();

    for i in 0..C {
        let s1 = lines.next().unwrap().chars().collect();
        let s2 = lines.next().unwrap().chars().collect();

        let result = -1;
        let output = format!("Case {}: {}", i, result);
    }
}
