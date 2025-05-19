use super::Solution;

impl Solution {
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        let (mut profit, mut previous) = (0, i32::MAX);
        for price in prices.iter() {
            profit += std::cmp::max(0, *price - previous);
            previous = *price;
        }
        profit
    }
}