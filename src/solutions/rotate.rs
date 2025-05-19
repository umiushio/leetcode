use super::Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        //该题还可以用反转法，即三次反转得到结果
        let n = nums.len() as i32;
        let k_ = k % n;
        let d = Self::gcd(n, k_) % n;
        if d == 0 {
            return
        }
        let m = n / d;
        for i in 0..d {
            let tmp = nums[i as usize];
            for j in 0..m-1 {
                nums[((i + (m - j) * k_) % n) as usize] = nums[((i + (m - j - 1) * k_) % n) as usize];
            }
            nums[((i + k_) % n) as usize] = tmp;
        }
    }

    fn gcd(a: i32, b: i32) -> i32 {
        // 迭代版
        let (mut a_, mut b_) = (a, b);
        while b_ != 0 {
            let tmp = b_;
            b_ = a_ % b_;
            a_ = tmp;
        }
        a_.abs()

        // 递归版
        // if b == 0 {
        //     a
        // } else {
        //     Self::gcd(b, a % b)
        // }
    }
}