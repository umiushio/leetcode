use super::Solution;

impl Solution {
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }
        let mut i = 2;
        for j in 2..n {
            if nums[j] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
        // if nums.is_empty() {
        //     return 0;
        // }
        // let (mut i, mut cnt) = (0, 0);
        // for j in 1..nums.len() {
        //     if nums[j] != nums[i] {
        //         nums[i + 1] = nums[j];
        //         i += 1;
        //         cnt = 0;
        //     } else {
        //         if cnt < 1 {
        //             nums[i + 1] = nums[j];
        //             i += 1;
        //         }
        //         cnt += 1;
        //     }
        // }
        // i as i32 + 1
    }
}