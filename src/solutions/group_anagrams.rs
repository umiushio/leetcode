use super::Solution;
use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 解法一：排序后的字节数组作为键（适合任意ASCII字符）
        // let mut anagram_map = HashMap::new();
        // for str in strs {
        //     let mut sorted_str = str.clone().into_bytes();
        //     sorted_str.sort_unstable();
        //     anagram_map.entry(sorted_str).or_insert_with(Vec::new).push(str);
        // }

        // 解法二：字符计数数组作为键（只适合小写字母）
        // let mut anagram_map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        // for str in strs {
        //     let mut counter = [0u8; 26];
        //     for byte in str.bytes() {
        //         counter[(byte - b'a') as usize] += 1;
        //     }
        //     anagram_map.entry(counter).or_insert_with(Vec::new).push(str);
        // }

        // 解法三：BTreeMap<char, usize>作为键（适合任意Unicode字符）
        let mut anagram_map: HashMap<BTreeMap<char, usize>, Vec<String>> = HashMap::new();
        for str in strs {
            let mut str_map: BTreeMap<char, usize> = BTreeMap::new();
            for ch in str.chars() {
                *str_map.entry(ch).or_insert(0) += 1;
            }
            anagram_map.entry(str_map).or_insert_with(Vec::new).push(str);
        }

        anagram_map.into_values().collect()
    }
}