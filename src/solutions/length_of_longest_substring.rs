use super::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // HashSet解法
        let mut max_length = 0;
        let mut left = 0;
        let mut byte_set: HashSet<u8> = HashSet::new();

        let byte_array: Vec<u8> = s.bytes().collect();
        for (right, right_byte) in byte_array.iter().enumerate() {
            if byte_set.contains(right_byte) {
                while byte_array[left] != *right_byte {
                    byte_set.remove(&byte_array[left]);
                    left += 1;
                }
                left += 1;
            } else {
                byte_set.insert(*right_byte);
                max_length = max_length.max(right - left + 1);
            }
        }
        max_length as i32
    }
}