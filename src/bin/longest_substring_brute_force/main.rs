use std::cmp::max;

fn main() {
    let longest = length_of_longest_substring("abcabcbb".to_string());
    assert_eq!(longest, 3i32);
    let longest = length_of_longest_substring("bbbbb".to_string());
    assert_eq!(longest, 1i32);
    let longest = length_of_longest_substring("pwwkew".to_string());
    assert_eq!(longest, 3i32);
    let longest = length_of_longest_substring("".to_string());
    assert_eq!(longest, 0);
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut res: i32 = 0;
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        for j in i..chars.len() {
            if non_repeat(&chars, i, j) {
                res = max(res, i32::try_from(j - i + 1).unwrap());
            }
        }
    }
    res
}

fn non_repeat(s: &Vec<char>, start: usize, end: usize) -> bool {
    let mut v = vec![0; 256];
    for k in start..end + 1 {
        let c_ascii = s[k as usize] as u8;
        v[c_ascii as usize] += 1;
        if v[c_ascii as usize] > 1 {
            return false;
        }
    }
    return true;
}
