use super::Solution;

impl Solution {
    pub fn cap_jump(nums: Vec<i32>) -> bool {
        assert!(!nums.is_empty());
        let mut max_jump_index = 0;
        let n = nums.len();
        for i in 0..n {
            max_jump_index = std::cmp::max(max_jump_index, i + nums[i] as usize);
            if max_jump_index >= n - 1 {
                return true;
            } else if max_jump_index == i {
                break;
            }
        }
        false
    }
}