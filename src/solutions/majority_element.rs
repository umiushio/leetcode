use super::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        let mut ret = nums.get(0).unwrap();
        let (mut i, mut cnt) = (1, 1);
        while i < size {
            if nums[i] == *ret {
                cnt += 1;
            } else {
                cnt -= 1;
            }
            if cnt + i >= size {
                break;
            } else if cnt == 0 {
                ret = nums.get(i + 1).unwrap();
                cnt = 1;
                i += 1;
            }
            i += 1;
        }
        *ret
    }
}