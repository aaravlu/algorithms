pub fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; n + 1];

    // 初始条件
    dp[0] = 1; // 空树算一种情况

    // 计算 dp[1] 到 dp[n]
    for i in 1..=n {
        for j in 1..=i {
            // dp[i] = Σ (dp[j-1] * dp[i-j])
            dp[i] += dp[j - 1] * dp[i - j];
        }
    }

    dp[n]
}
fn main() {}
