use super::Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        if n == 0 { return 0; }
        let mut asc = vec![1; n];
        let mut desc = vec![1; n];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                asc[i] = asc[i-1] + 1;
            }
        }

        for i in (0..n-1).rev() {
            if ratings[i] > ratings[i + 1] {
                desc[i] = desc[i+1] + 1;
            }
        }

        asc.iter().zip(desc.iter()).map(|(a, d)| a.max(d)).sum()

        // assert!(!ratings.is_empty());
        // let (mut asc, mut desc) = (0, 0);
        // let mut total = 1;
        // for i in 1..ratings.len() {
        //     if ratings[i] > ratings[i-1] {
        //         if desc > 0 {
        //             total += std::cmp::max(0, desc - asc);
        //             desc = 0;
        //             asc = 0;
        //         }
        //         asc += 1;
        //         total += asc + 1;
        //     } else if ratings[i] < ratings[i-1] {
        //         desc += 1;
        //         total += desc;
        //     } else {
        //         total += std::cmp::max(0, desc - asc) + 1;
        //         desc = 0;
        //         asc = 0;
        //     }
        // }
        // total + std::cmp::max(0, desc - asc)
    }
}