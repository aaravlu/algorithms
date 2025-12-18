pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let num_rows = num_rows as usize;

    // Build
    let mut dp = Vec::new();
    for i in 1..=num_rows {
        dp.push(vec![1; i]);
    }

    for i in 0..num_rows {
        for j in 1..i {
            dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
        }
    }
    dp
}

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn t() {}
}
