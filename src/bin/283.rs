pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut non_zero_index = 0;

    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[non_zero_index] = nums[i];
            non_zero_index += 1;
        }
    }

    for val in nums.iter_mut().skip(non_zero_index) {
        *val = 0;
    }
}

fn main() {}
