use super::Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        // 滑动数组
        assert!(!nums.is_empty());

        let n = nums.len();

        let mut sum = 0;
        let mut min_len = n + 1;
        let mut left = 0;
        for right in 0..n {
            sum += nums[right];
            while sum >= target {
                min_len = min_len.min(right - left + 1);
                if min_len == 1 {
                    return 1;
                }
                sum -= nums[left];
                left += 1;
            }
        }

        // let (mut left, mut right) = (0, 1);
        // let mut sum = nums[0];
        // let mut min_len = n + 1;
        // while left < n {
        //     if sum >= target {
        //         min_len = min_len.min(right - left);
        //         sum -= nums[left];
        //         left += 1;
        //     } else if right < n {
        //         sum += nums[right];
        //         right += 1;
        //     } else {
        //         break;
        //     }
        // }

        if min_len > n {
            0
        } else {
            min_len as i32
        }
    }
}