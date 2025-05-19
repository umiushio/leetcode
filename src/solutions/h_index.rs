use super::Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        // 二分法
        let (mut left, mut right) = (0, citations.len() as i32);
        while left < right {
            let mid = (left + right + 1) / 2;
            let count = citations.iter().filter(|&&v| v >= mid).count() as i32;
            if count >= mid {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
        
        // // 计数法
        // let n = citations.len();
        // let mut counters = vec![0; n + 1];
        // for citation in citations {
        //     counters[std::cmp::min(n, citation as usize)] += 1;
        // }
        // let mut h = n as i32;
        // let mut sum = 0;
        // for &counter in counters.iter().rev() {
        //     sum += counter;
        //     if sum >= h {
        //         break;
        //     }
        //     h -= 1;
        // }
        // h

        // // 排序法
        // let mut sorted_citations = citations.clone();
        // sorted_citations.sort_unstable();
        // let mut h = citations.len() as i32;
        // for citation in sorted_citations {
        //     if citation >= h {
        //         break;
        //     }
        //     h -= 1;
        // }
        // h
    }
}