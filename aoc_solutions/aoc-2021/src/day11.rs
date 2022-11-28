const WIDTH: usize = 10;
const HEIGHT: usize = 9;

pub fn solve_first(input: String) {
    let octopi: Vec<(usize, u32)> = input
        .lines()
        .collect::<String>()
        .chars()
        .map(|c| (0, c.to_digit(10).unwrap()))
        .collect();
    let n_steps = 100;
    let n_flashes = simulate_for(octopi, n_steps);
    println!("n_flashes: {n_flashes}");
}

fn simulate_for(mut octopi: Vec<(usize, u32)>, n_steps: i32) -> i32 {
    for step in 0..n_steps {
        for (step, energy_level) in octopi.iter_mut() {
            *energy_level += 1;
        }
    }
    0
}
