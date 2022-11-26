use std::cmp::Ord;

pub fn solve_first(input: String) {
    let crab_subs_h_pos = input.split(',').map(|s| s.trim().parse::<usize>().unwrap());
    let mut crab_subs: [i64; 2000] = [0; 2000];
    for h_pos in crab_subs_h_pos {
        crab_subs[h_pos] += 1;
    }
    let mut min = i64::MAX;
    for (h1, _) in crab_subs.iter().enumerate() {
        let mut sum = 0;
        for (h2, c2) in crab_subs.iter().enumerate() {
            let dist = (h2 as i64 - h1 as i64).abs();
            sum += dist * c2; // the distance between i1,i2
        }
        min = min.min(sum);
    }
    println!("{min}");
}

pub fn solve_second(input: String) {
    let crab_subs_h_pos = input.split(',').map(|s| s.trim().parse::<usize>().unwrap());
    let mut crab_subs: [i64; 2000] = [0; 2000];
    for h_pos in crab_subs_h_pos {
        crab_subs[h_pos] += 1;
    }
    let mut min = i64::MAX;
    for (h1, c1) in crab_subs.iter().enumerate() {
        let mut sum = 0;
        for (h2, c2) in crab_subs.iter().enumerate() {
            let dist = (h2 as i64 - h1 as i64).abs();
            let fuel_cost = fuel_cost(dist);
            sum += fuel_cost * c2; // the distance between i1,i2
        }
        min = min.min(sum)
    }
    println!("{min}");
}

fn fuel_cost(dist: i64) -> i64 {
    dist * (1 + dist) / 2
}
