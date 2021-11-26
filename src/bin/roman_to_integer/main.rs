use std::collections::HashMap;

#[test]
fn test_roman_to_int() {
    assert_eq!(roman_to_int("MMCMLXXXIX".to_string()), 2989);
}

fn roman_to_int(s: String) -> i32 {
    let roman_map: HashMap<char, i32> = HashMap::from([('M', 1000), ('D', 500), ('C', 100), ('L', 50), ('X', 10), ('V', 5), ('I', 1)]);
    let mut total = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut i: i32 = 0;
    while i < chars.len() as i32 - 1 {
        let cur = *roman_map.get(&chars[i as usize]).unwrap();
        let next = *roman_map.get(&chars[i as usize + 1]).unwrap();
        if cur < next {
            total += next - cur;
            i += 2;
        } else {
            total += cur;
            i += 1;
        }
    }
    total
}