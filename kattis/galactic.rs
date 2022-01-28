use std::{io, str::FromStr};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let mut str_lines = buffer.lines();
    let (W, N) = {
        let mut W_and_N = &str_lines
            .next()
            .expect("wtf, there was no line")
            .split_ascii_whitespace();
        let W: u32 = W_and_N.next().unwrap().parse().unwrap();
        let N: u32 = W_and_N.next().unwrap().parse().unwrap();
        (W, N)
    };
    println!("{} and {}", W, N);

    let mut lines: Vec<Line> = Vec::with_capacity(W as usize);
    for line in str_lines {
        let line_div: Line = {
            let numbers: Vec<f32> = line
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            println!("{:?}", numbers);
            let k = (numbers[3] - numbers[1]) / (numbers[2] - numbers[0]);
            let m = numbers[1];
            Line { k, m }
        };
        lines.push(line_div);
    }
    println!("lines {:?}", lines);
}

#[derive(Debug)]
struct Line {
    k: f32,
    m: f32,
}

impl ToString for Line {
    fn to_string(&self) -> String {
        String::from(format!("Line: y = {}x + {}", self.k, self.m))
    }
}
