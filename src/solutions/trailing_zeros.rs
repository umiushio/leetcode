use super::Solution;

impl Solution {
    pub fn trailing_zeros(n: i32) -> i32 {
        let mut zeros = 0;
        let mut n = n;
        while n > 0 {
            zeros += n / 5;
            n /= 5;
        }
        zeros
    }
}