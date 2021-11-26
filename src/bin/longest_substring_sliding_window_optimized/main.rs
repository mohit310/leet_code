use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let longest = length_of_longest_substring("abcabcbb".to_string());
    println!("{}", longest);
    let longest = length_of_longest_substring("bbbbb".to_string());
    println!("{}", longest);
    let longest = length_of_longest_substring("pwwkew".to_string());
    println!("{}", longest);
    let longest = length_of_longest_substring("".to_string());
    println!("{}", longest);
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut res: i32 = 0;
    let mut m: HashMap<char, i32> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    for j in 0..chars.len() {
        if m.contains_key(&chars[j]) {
            i = max(*m.get(&chars[j]).unwrap(), i);
        }
        res = max(res, j as i32 - i + 1);
        m.insert(chars[j], (j + 1) as i32);
    }
    res
}
