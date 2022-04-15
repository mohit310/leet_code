use std::cmp::max;

fn main() {
    let longest = length_of_longest_substring("abcabcbb".to_string());
    assert_eq!(longest, 3);
    let longest = length_of_longest_substring("bbbbb".to_string());
    assert_eq!(longest, 1);
    let longest = length_of_longest_substring("pwwkew".to_string());
    assert_eq!(longest, 3);
    let longest = length_of_longest_substring("".to_string());
    assert_eq!(longest, 0);
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut res: i32 = 0;
    let mut v = vec![0; 256];
    let chars: Vec<char> = s.chars().collect();
    let (mut left, mut right) = (0, 0);
    while right < chars.len() {
        let c_ascii = chars[right as usize] as u8;
        v[c_ascii as usize] += 1;
        while v[c_ascii as usize] > 1 {
            let l_ascii = chars[left as usize] as u8;
            v[l_ascii as usize] -= 1;
            left += 1;
        }
        res = max(res, (right - left + 1) as i32);
        right += 1;
    }
    res
}
