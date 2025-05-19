use super::Solution;

impl Solution {
    pub fn cas_jump2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut step = 0;
        let (mut current_jump_inf, mut max_jump_index) = (0, 0);
        for i in 0..n {
            if current_jump_inf >= n - 1 {
                break;
            }
            max_jump_index = std::cmp::max(max_jump_index, i + nums[i] as usize);
            if i == current_jump_inf {
                step += 1;
                current_jump_inf = max_jump_index;
            }
        }
        step
    }
}