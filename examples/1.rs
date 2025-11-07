fn main() {}

pub fn _two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    Vec::with_capacity(0)
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if target == nums[i] + nums[j] {
                return vec![i as i32, j as i32];
            }
        }
    }

    Vec::with_capacity(0)
}
