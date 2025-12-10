fn main() {}

pub fn _is_palindrome(x: i32) -> bool {
    match x {
        ..0 => false,
        0 => true,
        _ => {
            let mut num = x;
            let mut digits = Vec::new();

            while num > 0 {
                digits.push((num % 10) as u8);
                num /= 10;
            }

            let mut left = 0;
            let mut right = digits.len() - 1;

            while left < right {
                if digits[left] != digits[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }

            true
        }
    }
}

pub fn is_palindrome(x: i32) -> bool {
    match x {
        x if x < 0 => false,
        0 => true,
        _ => {
            let mut x = x;
            let mut vec = Vec::new();

            while x > 0 {
                vec.push(x % 10);
                x /= 10
            }

            let mut left = 0;
            let mut right = vec.len() - 1;

            if vec.is_empty() {
                return false;
            }

            while left < right {
                if vec[left] != vec[right] {
                    return false;
                }
                left += 1;
                right -= 1
            }
            true
        }
    }
}
