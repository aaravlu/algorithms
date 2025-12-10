pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();

    fn backtrack(
        start: usize,
        current: &mut Vec<i32>,
        nums: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        result.push(current.clone());

        for i in start..nums.len() {
            current.push(nums[i]);
            backtrack(i + 1, current, nums, result);
            current.pop();
        }
    }

    backtrack(0, &mut current, &nums, &mut result);
    result
}

fn main() {}
