fn main() {}

pub fn is_valid(s: String) -> bool {
    if !s.len().is_multiple_of(2) {
        return false;
    }

    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => return false,
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn test_valid_multiple_types() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_invalid_mismatch() {
        assert_eq!(is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_valid_nested() {
        assert_eq!(is_valid("([])".to_string()), true);
    }

    #[test]
    fn test_invalid_order() {
        assert_eq!(is_valid("([)]".to_string()), false);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(is_valid("".to_string()), true);
    }

    #[test]
    fn test_odd_length() {
        assert_eq!(is_valid("(()".to_string()), false);
    }

    #[test]
    fn test_only_left_brackets() {
        assert_eq!(is_valid("(((".to_string()), false);
    }

    #[test]
    fn test_only_right_brackets() {
        assert_eq!(is_valid(")))".to_string()), false);
    }

    #[test]
    fn test_complex_valid() {
        assert_eq!(is_valid("{[]}".to_string()), true);
    }
}
