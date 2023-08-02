pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .windows(2)
        .filter(|&w| w[0] < w[1])
        .map(|w| w[1] - w[0])
        .sum()
}

fn main() {
    // let input = vec![7, 1, 5, 3, 6, 4];
    let input = vec![7];
    let output = max_profit(input);
    println!("output: {:?}", output);
}
