use super::Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let n = nums.len();
        let (mut l, mut r) = (0, n);
        while l < r {
            if nums[l] == val {
                r -= 1;
                nums[l] = nums[r];
            } else {
                l += 1;
            }
        }
        l as i32
    }
}
