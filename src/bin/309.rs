pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    let n = prices.len();
    let mut dp = vec![vec![0; 3]; n];
    dp[0][0] = 0;
    dp[0][1] = -prices[0];
    dp[0][2] = i32::MIN;
    for i in 1..n {
        dp[i][0] = dp[i - 1][0].max(dp[i - 1][2]);
        dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] - prices[i]);
        dp[i][2] = dp[i - 1][1] + prices[i];
    }

    dp[n - 1][0].max(dp[n - 1][2])
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(max_profit(vec![1, 2, 3, 0, 2]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_profit(vec![1]), 0);
    }

    #[test]
    fn empty() {
        assert_eq!(max_profit(vec![]), 0);
    }

    #[test]
    fn no_profit() {
        assert_eq!(max_profit(vec![3, 2, 1]), 0);
    }

    #[test]
    fn multiple_transactions() {
        assert_eq!(max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 11);
    }
}
