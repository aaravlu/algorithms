// 283. 移动零
// 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
//
// 算法思路：双指针法
// - 使用两个指针：一个遍历指针i扫描整个数组，一个非零指针non_zero_index记录下一个非零元素应该放置的位置
// - 当遇到非零元素时，将其移动到non_zero_index位置，然后non_zero_index向前移动
// - 遍历完成后，将non_zero_index之后的所有位置都设为0

pub fn move_zeroes(nums: &mut Vec<i32>) {
    // 使用双指针法：一个指针遍历数组，另一个指针指向下一个非零元素应该放置的位置
    let mut non_zero_index = 0; // 非零指针：记录下一个非零元素应该放置的位置

    // 遍历指针i：从头到尾扫描整个数组
    for i in 0..nums.len() {
        // 如果当前元素不为0，说明需要保留这个元素
        if nums[i] != 0 {
            // 将非零元素移动到non_zero_index位置
            // 这样所有非零元素都会被紧凑地排列在数组的前面
            nums[non_zero_index] = nums[i];
            // 非零指针向前移动，为下一个非零元素准备位置
            non_zero_index += 1;
        }
    }

    // 遍历完成后，所有非零元素都已经移动到数组前面
    // 现在将剩余的位置（从non_zero_index到数组末尾）都填充为0
    // for i in non_zero_index..nums.len() {
    for val in nums.iter_mut().skip(non_zero_index) {
        *val = 0;
    }
}

fn main() {}
