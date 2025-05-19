use super::Solution;

impl Solution {
    pub fn my_sqrt(n: i32) -> i32 {
        let (mut x, mut y) = (1, n);
        while (x - y).abs() > 1 {
            x += (y - x) / 2;
            y = n / x;
        }

        x.min(y)
    }
}