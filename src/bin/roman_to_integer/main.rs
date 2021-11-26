use std::collections::HashMap;

fn main() {
    assert_eq!(roman_to_int("MMCMLXXXIX".to_string()), 2989);
    assert_eq!(roman_to_int("III".to_string()), 3);
}

fn roman_to_int(s: String) -> i32 {
    let mut roman_map: HashMap<char, i32> = HashMap::new();
    roman_map.insert('M', 1000);
    roman_map.insert('D', 500);
    roman_map.insert('C', 100);
    roman_map.insert('L', 50);
    roman_map.insert('X', 10);
    roman_map.insert('V', 5);
    roman_map.insert('I', 1);

    let chars: Vec<char> = s.chars().collect();
    let last_sym = chars[chars.len() - 1];
    let mut last_val = *roman_map.get(&last_sym).unwrap();
    let mut total = last_val;
    let mut i: i32 = chars.len() as i32 - 2;
    while i >= 0 {
        let cur_symbol = chars[i as usize];
        let cur_val = *roman_map.get(&cur_symbol).unwrap();
        if cur_val < last_val {
            total -= cur_val;
        } else {
            total += cur_val;
        }
        last_val = cur_val;
        i -= 1;
    }
    total
}
