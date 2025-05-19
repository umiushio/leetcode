use super::Solution;
use std::collections::HashMap;

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct Line {
//     // 题目中的任何直线都可以用以下方式表示 ax + by = c (a, b, c均为整数)
//     // 为确保唯一性，规定 gcd(a, b) = 1 
//     // 由于 gcd 算法不固定输出为非负数的情况下，(a/d, b/d)是不会出现(-a/d, -b/d)的
//     a_: i32,
//     b_: i32,
//     // c_: i64,
// }

// impl Line {
//     pub fn new(p: &[i32], q: &[i32]) -> Self {
//         assert!(p.len() == 2 && q.len() == 2);
//         assert!(p[0] != q[0] || p[1] != q[1], "p and q should not be the same point");
//         let d = Self::gcd(q[1] - p[1], p[0] - q[0]);
//         let a_ = (q[1] - p[1]) / d;
//         let b_ = (p[0] - q[0]) / d;
//         // let c_ = (p[0] as i64) * (a_ as i64) + (p[1] as i64) * (b_ as i64);
//         // Self { a_, b_, c_ }
//         Self { a_, b_ }
//     }

//     fn gcd(mut a: i32, mut b: i32) -> i32 {
//         while b != 0 {
//             let tmp = b;
//             b = a % b;
//             a = tmp;
//         }
//         a
//     }
// }

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n <= 2 { return n as i32; }

        let mut max_count = 1;
        for (i, p) in points.iter().enumerate() {
            if max_count >= n - i || max_count > n / 2 {
                break;
            }
            let mut slope_map: HashMap<(i32, i32), usize> = HashMap::new();
            for q in points.iter().skip(i + 1) {
                let (dx, dy) = (p[0] - q[0], p[1] - q[1]);
                let d = gcd(dx, dy);
                // 归一化实际上已经有保证，虽然不能保证dx或dy一定>=0，但是不会出现对偶元组
                // 即如果存在(a, b), 则绝对不会出现(-a, -b)
                *slope_map.entry((dx / d, dy / d)).or_insert(1) += 1;
            }
            if let Some(&max_val) = slope_map.values().max() {
                max_count = max_count.max(max_val);
            }
        }

        max_count as i32
    }
}

// 作为模块私有自由函数
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    assert_ne!(a, 0, "a and b should not be the same");
    a
}