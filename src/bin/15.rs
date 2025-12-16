use std::io::{self, Read};

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut res: Vec<Vec<i32>> = Vec::new();
    if n < 3 {
        return res;
    }

    nums.sort();

    for i in 0..n - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        if nums[i] > 0 {
            break;
        }

        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum == 0 {
                res.push(vec![nums[i], nums[left], nums[right]]);

                let left_val = nums[left];
                while left < right && nums[left] == left_val {
                    left += 1;
                }

                let right_val = nums[right];
                while left < right && nums[right] == right_val {
                    right -= 1;
                }
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    res
}

// 简单的输入输出主函数，方便本地测试：
// 输入：一行，用空格分隔的整数
// 输出：所有三元组，每行一个 "a b c"（已排序并去重）
fn main() {
    // 读取整行输入
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let mut ans = three_sum(nums);

    // 为了输出稳定性，对结果排序（按字典序）
    ans.sort();

    for triplet in ans {
        if triplet.len() == 3 {
            println!("{} {} {}", triplet[0], triplet[1], triplet[2]);
        }
    }
}
