use super::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        //似乎用if/else会比max/min耗时更多
        let (mut profit, mut min_price) = (0, i32::MAX);
        for price in prices.iter() {
            min_price = std::cmp::min(min_price, *price);
            profit = std::cmp::max(profit, *price - min_price);
        }
        profit

        // assert!(!prices.is_empty());
        // let mut profit = 0;
        // let (mut min_price, mut max_price) = (prices[0], prices[0]);
        // for price in prices.iter() {
        //     if *price > max_price {
        //         max_price = *price;
        //         profit = std::cmp::max(profit, max_price - min_price);
        //     } else if *price < min_price {
        //         min_price = *price;
        //         max_price = *price;
        //     }
        // }
        // profit
    }
}