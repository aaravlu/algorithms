use std::cmp;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }

    let mut min_price = prices[0];
    let mut max_profit = 0;

    for price in &prices[1..] {
        max_profit = cmp::max(max_profit, price - min_price);
        min_price = cmp::min(min_price, *price);
    }

    max_profit
}

pub fn _max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }

    let mut min_price = prices[0];
    let mut max_profit = 0;

    for price in &prices[1..] {
        max_profit = max_profit.max(price - min_price);
        min_price = (*price).min(min_price);
    }

    max_profit
}

fn main() {}
