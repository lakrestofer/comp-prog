pub fn solve_first(input: String) {
    let mut rations = input
        .split("\n\n")
        .map(|rations| {
            rations
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .sum()
        })
        .collect::<Vec<u32>>();
    rations.sort();
    println!("first: {}", rations.pop().unwrap());
}

pub fn solve_second(input: String) {
    let mut rations = input
        .split("\n\n")
        .map(|rations| {
            rations
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .sum()
        })
        .collect::<Vec<u32>>();
    rations.sort();
    let mut sum = 0;
    for _ in 0..3 {
        sum += rations.pop().unwrap();
    }
    println!("second: {sum}");
}
