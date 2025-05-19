use super::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        assert!(n >= 2);
        let mut products = vec![0; nums.len()];
        let mut product = 1;
        products[0] = 1;
        for i in 0..n-1 {
            product *= nums[i];
            products[i + 1] = product;
        }
        product = 1;
        for i in (1..n).rev() {
            product *= nums[i];
            products[i - 1] *= product;
        }
        products
    }
}