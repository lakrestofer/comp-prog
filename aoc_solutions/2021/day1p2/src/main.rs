fn main() {
    let input: Vec<i64> = std::fs::read_to_string("input.txt")
        .expect("could not read input")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let sums: Vec<i64> = input[..].windows(3).map(|x| x.iter().sum()).collect();
    let count: usize = sums[..]
        .windows(2)
        .filter(|slice| match slice {
            &&[first, second] => first < second,
            _ => unreachable!(),
        })
        .count();
    println!("{}", count);
}
