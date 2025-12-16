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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome_empty() {
        assert_eq!(longest_palindrome("".to_string()), "");
    }

    #[test]
    fn test_longest_palindrome_single_char() {
        assert_eq!(longest_palindrome("a".to_string()), "a");
    }

    #[test]
    fn test_longest_palindrome_two_same() {
        assert_eq!(longest_palindrome("aa".to_string()), "aa");
    }

    #[test]
    fn test_longest_palindrome_two_different() {
        let result = longest_palindrome("ab".to_string());
        assert!(result == "a" || result == "b");
    }

    #[test]
    fn test_longest_palindrome_odd_length() {
        assert_eq!(longest_palindrome("aba".to_string()), "aba");
    }

    #[test]
    fn test_longest_palindrome_even_length() {
        assert_eq!(longest_palindrome("abba".to_string()), "abba");
    }

    #[test]
    fn test_longest_palindrome_no_palindrome() {
        let result = longest_palindrome("abc".to_string());
        assert!(result.len() == 1);
    }

    #[test]
    fn test_longest_palindrome_longer() {
        let result = longest_palindrome("babad".to_string());
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn test_expand_around_center_single() {
        let chars = vec!['a', 'b', 'a'];
        assert_eq!(expand_around_center(&chars, 1, 1), 1);
    }

    #[test]
    fn test_expand_around_center_odd() {
        let chars = vec!['a', 'b', 'a'];
        assert_eq!(expand_around_center(&chars, 1, 1), 3);
    }

    #[test]
    fn test_expand_around_center_even() {
        let chars = vec!['a', 'a'];
        assert_eq!(expand_around_center(&chars, 0, 1), 2);
    }
}
