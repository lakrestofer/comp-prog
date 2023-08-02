pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy_price = &prices[0];
    let mut profit = 0;

    let days = prices.len();
    let mut day = 1;
    while day < days {
        let new_price = &prices[day];
        let new_profit = new_price - buy_price;
        if buy_price < new_price {
            if profit < new_profit {
                profit = new_profit;
            }
        } else {
            buy_price = new_price;
        }
        day += 1;
    }
    profit
}

fn main() {
    let input = vec![7, 1, 5, 3, 6, 4];
    let output = max_profit(input);
    println!("output: {:?}", output);
}
