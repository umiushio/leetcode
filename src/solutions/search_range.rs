use super::Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        let (mut left, mut right) = (0, n);
        while left + 1 < right {
            let mid = (left + right - 1) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid;
            } else {
                return vec![(left + Self::binary_search(&nums[left..mid+1], target, 1)) as i32,
                    (mid + Self::binary_search(&nums[mid..right], target, 2)) as i32];
            }
        }
        if right != 0 && left != n && nums[left] == target {
            return vec![left as i32, left as i32];
        } 
        vec![-1, -1]
        
        // let n = nums.len();
        // let (mut left, mut right) = (0, n);
        // while left + 1 < right {
        //     let mid = (left + right - 1) / 2;
        //     if nums[mid] < target {
        //         left = mid + 1;
        //     } else if nums[mid] > target {
        //         right = mid;
        //     } else {
        //         right = mid + 1;
        //     }
        // }
        // if right == 0 || left == n || nums[left] != target {
        //     return vec![-1, -1];
        // } 
        // let mut range = vec![left as i32];
        // right = n;
        // while left + 1 < right  {
        //     let mid = (left + right) / 2;
        //     if nums[mid] < target {
        //         left = mid + 1;
        //     } else if nums[mid] > target {
        //         right = mid;
        //     } else {
        //         left = mid;
        //     }
        // }
        // range.push(left as i32);
        // range
    }

    fn binary_search(nums: &[i32], target: i32, mode: usize) -> usize {
        let n = nums.len();
        let (mut left, mut right) = (0, n);
        while left + 1 < right {
            let mid = (left + right + mode) / 2 - 1;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid;
            } else {
                if mode == 1 {
                    right = mid + 1;
                } else if mode == 2 {
                    left = mid;
                } else {
                    return mid;
                }
            }
        }
        left
    }
}