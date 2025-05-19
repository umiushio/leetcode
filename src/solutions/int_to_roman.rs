use super::Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        assert!(num > 0 && num < 4000);
        let int_to_roman_vec = ['I','V', 'X', 'L', 'C', 'D', 'M'];
        let mut s = String::new();
        let mut index = 0;
        let mut mut_num = num;
        while mut_num > 0 {
            let digit = mut_num % 10;
            if digit == 9 {
                s.push(int_to_roman_vec[index + 2]);
                s.push(int_to_roman_vec[index]);
            } else if digit == 4 {
                s.push(int_to_roman_vec[index + 1]);
                s.push(int_to_roman_vec[index]);
            } else if digit >= 5 {
                for _ in 0..digit - 5 {
                    s.push(int_to_roman_vec[index]);
                }
                s.push(int_to_roman_vec[index + 1]);
            } else {
                for _ in 0..digit {
                    s.push(int_to_roman_vec[index]);
                }
            }
            index += 2;
            mut_num /= 10;
        }
        s.chars().rev().collect()
    }
}