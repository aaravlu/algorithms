pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }
    let mut dp = vec![0; n + 1];
    dp[1] = nums[0];
    for i in 2..=n {
        dp[i] = dp[i - 1].max(nums[i - 1] + dp[i - 2]);
    }
    dp[n]
}

fn main() {
    let nums1 = vec![1, 2, 3, 1];
    println!("Max rob for [1,2,3,1]: {}", rob(nums1));
    let nums2 = vec![2, 7, 9, 3, 1];
    println!("Max rob for [2,7,9,3,1]: {}", rob(nums2));
}
