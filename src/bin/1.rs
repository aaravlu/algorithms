fn main() {}

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

#[cfg(test)]
mod test {
    #[test]
    fn t() {
        use crate::two_sum;
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
