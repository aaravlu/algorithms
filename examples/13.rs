fn main() {}

pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;

    fn trans(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!(),
        }
    }

    let nums = s.chars().map(trans).collect::<Vec<_>>();

    for i in 0..nums.len() {
        if i + 1 < nums.len() && nums[i] < nums[i + 1] {
            result += -nums[i];
        } else {
            result += nums[i];
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("IX".to_string()), 9);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(roman_to_int("XL".to_string()), 40);
        assert_eq!(roman_to_int("XC".to_string()), 90);
        assert_eq!(roman_to_int("CD".to_string()), 400);
        assert_eq!(roman_to_int("CM".to_string()), 900);
    }
}
