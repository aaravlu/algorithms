use std::cmp::max;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut last = vec![0u16; 256];
    let bytes = s.as_bytes();

    let mut left = 0;
    let mut max_len = 0;

    for (right, b) in bytes.iter().enumerate() {
        let idx = *b as usize;

        if last[idx] > left {
            left = last[idx];
        }

        last[idx] = right as u16 + 1;

        let window_len = right as u16 - left + 1;
        max_len = max(max_len, window_len);
    }

    max_len as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    fn example1() {
        let s = "abcabcbb".to_string();
        assert_eq!(length_of_longest_substring(s), 3);
    }

    #[test]
    fn example2() {
        let s = "bbbbb".to_string();
        assert_eq!(length_of_longest_substring(s), 1);
    }

    #[test]
    fn example3() {
        let s = "pwwkew".to_string();
        // 最长无重复子串是 "wke"，长度为 3
        assert_eq!(length_of_longest_substring(s), 3);
    }

    #[test]
    fn empty_string() {
        let s = "".to_string();
        assert_eq!(length_of_longest_substring(s), 0);
    }

    #[test]
    fn single_char() {
        let s = "a".to_string();
        assert_eq!(length_of_longest_substring(s), 1);
    }

    #[test]
    fn all_unique() {
        let s = "abcdef".to_string();
        assert_eq!(length_of_longest_substring(s), 6);
    }

    #[test]
    fn repeated_pattern() {
        let s = "abba".to_string();
        // 最长无重复子串是 "ab" 或 "ba"，长度为 2
        assert_eq!(length_of_longest_substring(s), 2);
    }
}
