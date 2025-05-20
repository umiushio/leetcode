use super::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length = 0;
        for ch in s.bytes().rev() {
            if ch == b' ' {
                if length > 0 {
                    break;
                }
            } else {
                length += 1;
            }
        }
        length
    }
}