use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();

    let T = lines.next().unwrap().parse().unwrap();

    for _ in 0..T {
        let n_candidates = lines.next().unwrap().parse().unwrap();
        let mut total = 0;
        let mut votes = Vec::with_capacity(n_candidates);

        for _ in 0..n_candidates {
            let vote: u32 = lines.next().unwrap().parse().unwrap();
            total += vote;
            votes.push(vote);
        }

        let (winner, max): (usize, u32) = votes
            .iter()
            .enumerate()
            .map(|(c, v)| (c + 1, *v))
            .max()
            .unwrap();

        let is_valid = votes.iter().filter(|v| **v == max).count() == 1;

        if !is_valid {
            println!("no winner");
        } else {
            if max > (total / 2) {
                println!("majority winner {}", winner);
            } else {
                println!("minority winner {}", winner);
            }
        }
    }
}
