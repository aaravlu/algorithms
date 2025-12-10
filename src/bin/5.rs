fn main() {}

pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut end = 0;

    for i in 0..chars.len() {
        // 检查奇数长度的回文（以单个字符为中心）
        let len1 = expand_around_center(&chars, i, i);
        // 检查偶数长度的回文（以两个相同字符为中心）
        let len2 = expand_around_center(&chars, i, i + 1);

        let len = len1.max(len2);

        if len > end - start {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }

    chars[start..=end].iter().collect()
}

fn expand_around_center(chars: &[char], left: usize, right: usize) -> usize {
    let mut l = left as i32;
    let mut r = right as i32;
    let n = chars.len() as i32;

    while l >= 0 && r < n && chars[l as usize] == chars[r as usize] {
        l -= 1;
        r += 1;
    }

    (r - l - 1) as usize
}
