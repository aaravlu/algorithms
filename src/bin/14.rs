fn main() {}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut ret = String::with_capacity(0);

    let mut min_str = strs[0].clone();
    for (i, str) in strs.iter().enumerate() {
        if str.len() < min_str.len() {
            min_str = strs[i].clone();
        }
    }
    let min_str_len = min_str.len();

    for i in 0..min_str_len {
        if strs
            .iter()
            .all(|str| str.starts_with(&min_str[..min_str_len - i]))
        {
            ret.push_str(&min_str[..min_str_len - i]);
            break;
        }
    }

    ret
}
