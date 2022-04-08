use std::collections::HashMap;

fn main() {
    assert_eq!(roman_to_int("MMCMLXXXIX".to_string()), 2989);
    assert_eq!(roman_to_int("III".to_string()), 3);
}

fn roman_to_int(s: String) -> u32 {
    let roman_map: HashMap<_, _> = [('M', 1000u32), ('D', 500u32), ('C', 100u32), ('L', 50u32), ('X', 10u32), ('V', 5u32), ('I', 1u32)].into();
    let length = s.len();
    let chars: Vec<char> = s.chars().collect();
    let last_sym = chars[length - 1];
    let mut last_val = roman_map.get(&last_sym).unwrap();
    let mut total = *last_val;
    for i in (0..chars.len() - 1).rev() {
        let cur_symbol = chars[i];
        let cur_val = roman_map.get(&cur_symbol).unwrap();
        if cur_val < last_val {
            total -= cur_val;
        } else {
            total += cur_val;
        }
        last_val = cur_val;
    }
    total.clone()
}
