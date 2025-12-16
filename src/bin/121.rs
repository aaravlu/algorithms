pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    if n < 2 {
        return 0;
    }

    let mut dp = vec![0; n];
    let mut min_price = prices[0];

    for i in 1..n {
        min_price = min_price.min(prices[i]);
        dp[i] = dp[i - 1].max(prices[i] - min_price);
    }

    dp[n - 1]
}

fn main() {}
