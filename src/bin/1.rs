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
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_basic() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_multiple_pairs() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_two_sum_no_solution() {
        let nums = vec![1, 2, 3];
        let target = 10;
        let result = two_sum(nums, target);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_two_sum_negative_numbers() {
        let nums = vec![-1, -2, -3, -4, -5];
        let target = -8;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_two_sum_zero() {
        let nums = vec![0, 4, 3, 0];
        let target = 0;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 3]);
    }

    #[test]
    fn test_two_sum_single_element() {
        let nums = vec![1];
        let target = 2;
        let result = two_sum(nums, target);
        assert_eq!(result, Vec::<i32>::new());
    }
}
