pub fn climb_stairs(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    let mut a = 1;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
pub fn _climb_stairs(n: i32) -> i32 {
    // dp[0] = 1 dp[1] = 1
    // dp[n] = dp[n-2] + dp[n-1]
    let n = n as usize;
    let mut dp = vec![1; n + 1];

    for i in 2..=n {
        dp[i] = dp[i - 2] + dp[i - 1]
    }

    dp[n]
}

fn main() {
    println!("Ways to climb 2 stairs: {}", _climb_stairs(2));
    println!("Ways to climb 3 stairs: {}", _climb_stairs(3));
}
