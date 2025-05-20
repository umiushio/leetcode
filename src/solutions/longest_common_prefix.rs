use super::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() { return String::from(""); }

        let prefix = strs[0].as_bytes();
        let mut max_length = strs[0].len();
        for str in strs.iter().skip(1) {
            max_length = max_length.min(str.len());
            for (i, byte) in str[0..max_length].bytes().enumerate() {
                if prefix[i] != byte {
                    max_length = i;
                    break;
                }
            }
            if max_length == 0 {
                return String::from("");
            }
        }
        String::from_utf8(prefix[0..max_length].to_vec()).unwrap()

        // let first_str_bytes: Vec<u8> = strs[0].bytes().collect();
        // let mut max_length = first_str_bytes.len();
        // for str in strs[1..].iter() {
        //     max_length = max_length.min(str.len());
        //     if max_length == 0 {
        //         break;
        //     }

        //     for (i, byte) in str[0..max_length].bytes().enumerate() {
        //         if first_str_bytes[i] != byte {
        //             max_length = i;
        //             break;
        //         }
        //     }
        // }

        // if max_length == 0 {
        //     String::from("")
        // } else {
        //     String::from_utf8(first_str_bytes[0..max_length].to_vec()).unwrap()
        // }
    }
}