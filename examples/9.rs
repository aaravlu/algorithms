fn main() {}

pub fn is_palindrome(x: i32) -> bool {
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
