use super::Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut i, mut j) = (m, n);
        for k in (0..(m + n)).rev() {
            if j <= 0 {
                break;
            }
            nums1[k as usize] = if i <= 0 || nums1[i as usize - 1] < nums2[j as usize - 1] {
                j -= 1;
                nums2[j as usize]
            } else {
                i -= 1;
                nums1[i as usize]
            };
        }
        // while实现，实际上内存消耗会更多，更推荐用for迭代器进行循环迭代
        // let mut k = (m + n) as usize;
        // while j > 0 {
        //     k -= 1;
        //     nums1[k] = if i <= 0 || nums1[i as usize - 1] < nums2[j as usize - 1] {
        //         j -= 1;
        //         nums2[j as usize]
        //     } else {
        //         i -= 1;
        //         nums1[i as usize]
        //     };
        // }
    }
}
