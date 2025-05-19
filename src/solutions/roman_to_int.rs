use super::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_map: HashMap<char, i32> = [
            ('I', 1), ('V', 5), ('X', 10), ('L', 50),
            ('C', 100), ('D', 500), ('M', 1000)
        ].iter().cloned().collect();

        let mut total = 0;
        let mut prev = 0;
        for ch in s.chars().rev() {
            let curr = roman_map[&ch];
            total += if curr >= prev {
                curr
            } else {
                -curr
            };
            prev = curr
        }
        total
    }
}