use super::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }
        let mut counter: HashMap<char, usize> = HashMap::new();
        for ch in s.chars() {
            *counter.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            if let Some(count) = counter.get_mut(&ch) {
                *count -= 1;
                if *count == 0 {
                    counter.remove(&ch);
                }
            } else {
                return false;
            }

            // if let Some(count) = counter.get_mut(&ch) {
            //     if *count == 0 {
            //         return false;
            //     }
            //     *count -= 1;
            // } else {
            //     return false;
            // }

            // let entry = counter.entry(ch).or_insert(0);
            // if *entry == 0 {
            //     return false;
            // } else {
            //     *entry -= 1;
            // }

            // if let Some(&count) = s_counter.get(&ch) {
            //     if count == 0 {
            //         return false;
            //     } else {
            //         s_counter.insert(ch, count - 1);
            //     }
            // } else {
            //     return false;
            // }
        }
        true
    }
}