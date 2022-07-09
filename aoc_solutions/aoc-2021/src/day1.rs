pub fn solve_first(input: String) {
    let lines: Vec<u32> = input.lines().map(|s| s.parse().unwrap()).collect();
    let mut counter = 0;
    for window in lines.windows(2) {
        match window {
            &[n1, n2] => {
                if n1 < n2 {
                    counter += 1
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{counter}");
}
pub fn solve_second(input: String) {
    let lines: Vec<u32> = input.lines().map(|s| s.parse().unwrap()).collect();
    let mut counter = 0;
    for window in lines.windows(4) {
        match window {
            &[n1, n2, n3, n4] => {
                if n1 + n2 + n3 < n2 + n3 + n4 {
                    counter += 1
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{counter}");
}
