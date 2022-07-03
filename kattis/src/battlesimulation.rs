use std::collections::HashMap;
use std::io::{stdin, Read};

pub fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut output = String::with_capacity(buffer.len());

    let c = buffer.trim().chars().collect::<Vec<char>>();

    let mut i = 0;

    while i < c.len() {
        if (i + 2 < c.len() && c[i] != c[i + 1] && c[i] != c[i + 2] && c[i + 1] != c[i + 2]) {
            output.push('C');
            i += 2;
        } else if c[i] == 'R' {
            output.push('S');
        } else if c[i] == 'B' {
            output.push('K');
        } else if c[i] == 'L' {
            output.push('H');
        }
        i += 1;
    }
    println!("{}", output);
}
