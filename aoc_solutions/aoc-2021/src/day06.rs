use std::mem::swap;
use std::result::Result;
use std::str::FromStr;

const N_DAYS: usize = 80;

pub fn solve_first(input: String) {
    println!("n_fish after 80 days:");
    solve(input, 80);
}

pub fn solve_second(input: String) {
    println!("n_fish after 256 days:");
    solve(input, 256);
}

fn solve(input: String, n_days: usize) {
    let mut fish = [0; 9];
    let numbers: Vec<usize> = input
        .split(',')
        .map(|s| s.trim())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("{:?}", numbers);
    for number in numbers {
        fish[number] += 1;
    }

    for _ in 0..n_days {
        simulate_day(&mut fish);
    }

    println!("{}", count_fish(&fish));
}

fn simulate_day(fs: &mut [usize; 9]) {
    let mut new = [0; 9];
    new[8] += fs[0];
    new[6] += fs[0];
    for i in 0..=7 {
        new[i] += fs[i + 1];
    }
    swap(fs, &mut new);
}

fn count_fish(fs: &[usize; 9]) -> usize {
    fs.iter().sum()
}
