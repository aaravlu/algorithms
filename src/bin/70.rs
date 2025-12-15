pub fn climb_stairs(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![1; n + 1];

    for i in 2..=n {
        dp[i] = dp[i - 2] + dp[i - 1]
    }

    dp[n]
}

fn main() {
    println!("Ways to climb 2 stairs: {}", climb_stairs(2));
    println!("Ways to climb 3 stairs: {}", climb_stairs(3));
}
