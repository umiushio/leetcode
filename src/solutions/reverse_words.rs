use super::Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words = Vec::new();
        let mut start = 0;
        for (i, ch) in s.chars().enumerate() {
            if ch == ' ' {
                if start < i {
                    words.push(&s[start..i]);
                }
                start = i + 1;
            }
        }
        if start < s.len() {
            words.push(&s[start..]);
        }
        words.reverse();
        words.join(" ")
    }
}