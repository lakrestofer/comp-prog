pub fn solve_first(input: String) {
    let count = input
        .lines()
        .map(|line| parse_ranges(line))
        .filter(|((s1, e1), (s2, e2))| range_contains_range(*s1, *e1, *s2, *e2))
        .count();
    println!("{count}");
}

fn parse_ranges(line: &str) -> ((usize, usize), (usize, usize)) {
    let (lr, hr) = line.split_once(",").unwrap();
    let (s1, e1) = lr.split_once("-").unwrap();
    let (s2, e2) = hr.split_once("-").unwrap();
    let lr: (usize, usize) = (s1.parse().unwrap(), e1.parse().unwrap());
    let hr: (usize, usize) = (s2.parse().unwrap(), e2.parse().unwrap());
    (lr, hr)
}

fn range_contains_range(s1: usize, e1: usize, s2: usize, e2: usize) -> bool {
    s1 <= s2 && e2 <= e1 || s2 <= s1 && e1 <= e2
}
