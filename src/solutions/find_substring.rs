use super::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() {
            return vec![];
        }

        let (word_count, word_len, s_len) = (words.len(), words[0].len(), s.len());
        let total_len = word_count * word_len;
        if s_len < total_len { return vec![]; }

        let mut counter = HashMap::new();
        for word in words {
            *counter.entry(word).or_insert(0) += 1;
        }

        let mut ret = Vec::new();
        for i in 0..word_len {
            let mut left = i;
            while left <= s_len - total_len {
                let mut success = true;
                let mut current_counter = HashMap::new();
                for j in (0..word_count).rev() {
                    let word = &s[(left + j * word_len)..(left + (j + 1) * word_len)];
                    if counter.contains_key(word) {
                        let entry = current_counter.entry(word).or_insert(0);
                        if *entry == counter[word] {
                            left += (j + 1) * word_len;
                            success = false;
                            break;
                        }
                        *entry += 1;
                    } else {
                        left += (j + 1) * word_len;
                        success = false;
                        break;
                    }
                }
                if success {
                    loop {
                        ret.push(left as i32);
                        left += word_len;
                        if left > s_len - total_len || &s[left - word_len..left] != &s[(left - word_len + total_len)..(left + total_len)] {
                            break;
                        }
                    }
                }
            }
        }

        ret
    }
}